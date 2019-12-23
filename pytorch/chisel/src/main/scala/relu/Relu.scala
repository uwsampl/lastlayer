package relu

import chisel3._
import chisel3.util._

case class ReluConfig() {
  val xLen = 1
  val opDataWidth = 8
  val memDepth = 1024
  val memDataWidth = xLen * opDataWidth
  val memAddrWidth = log2Ceil(memDepth)
  val cycleCounterWidth = 32
}

class Op(implicit config: ReluConfig) extends Module {
  val io = IO(new Bundle {
    val a = Input(UInt(config.opDataWidth.W))
    val y = Output(UInt(config.opDataWidth.W))
  })
  io.y := Mux(io.a.asSInt > 0.S, io.a, 0.U)
}

class VecOp(implicit config: ReluConfig) extends Module {
  val io = IO(new Bundle {
    val in = Input(UInt(config.memDataWidth.W))
    val out = Output(UInt(config.memDataWidth.W))
  })
  val in = Wire(Vec(config.xLen, UInt(config.opDataWidth.W)))
  val out = Wire(Vec(config.xLen, UInt(config.opDataWidth.W)))
  val op = Seq.fill(config.xLen){ Module(new Op) }

  in := io.in.asTypeOf(in)

  Seq.tabulate(config.xLen){ i =>
    op(i).io.a := in(i)
    out(i) := op(i).io.y
  }

  io.out := out.asTypeOf(io.out)
}

class Exit extends BlackBox with HasBlackBoxInline {
  val io = IO(new Bundle{
    val en = Input(Bool())
  })
  setInline("Exit.v",
    s"""
      |module Exit(
      |  input en
      |);
      |always @* begin
      |  if (en) begin
      |    $$finish;
      |  end
      |end
      |endmodule
    """.stripMargin)
}

class Relu(implicit config: ReluConfig) extends Module {
  val io = IO(new Bundle{
    // these memory ports prevent optimize memory away
    val wen = Input(Bool())
    val waddr = Input(UInt(config.memAddrWidth.W))
    val wdata = Input(UInt(config.memDataWidth.W))
    val ren = Input(Bool())
    val raddr = Input(UInt(config.memAddrWidth.W))
    val rdata = Output(UInt(config.memDataWidth.W))
  })
  val exit = Module(new Exit)
  val vop = Module(new VecOp)
  val rmem = SyncReadMem(config.memDepth, UInt(config.memDataWidth.W))
  val wmem = SyncReadMem(config.memDepth, UInt(config.memDataWidth.W))
  val launch = RegInit(false.B)
  val finish = RegInit(false.B)
  val cycle = RegInit(0.U(config.cycleCounterWidth.W))
  val counter = RegInit(0.U(config.memAddrWidth.W))
  val length = RegInit(0.U(config.memAddrWidth.W))
  val raddr = RegInit(0.U(config.memAddrWidth.W))
  val waddr = RegInit(0.U(config.memAddrWidth.W))

  val sIdle :: sRead :: sWrite :: sDone :: Nil = Enum(4)
  val state = RegInit(sIdle)

  switch(state) {
    is(sIdle) {
      when(launch) {
        state := sRead
      }
    }
    is(sRead) {
      state := sWrite
    }
    is(sWrite) {
      when (counter === length) {
        state := sDone
      } .otherwise {
        state := sRead
      }
    }
    is(sDone) {
      state := sDone
    }
  }

  when (state === sIdle) {
    cycle := 0.U
  } .elsewhen (state =/= sDone) {
    cycle := cycle + 1.U
  }

  when (state === sIdle) {
    counter := 0.U
  } .elsewhen (state === sRead) {
    counter := counter + 1.U
  }

  when (state === sRead) {
    raddr := raddr + 1.U
  }

  when (state === sWrite) {
    waddr := waddr + 1.U
  }

  finish := state === sDone
  exit.io.en := finish

  val ren = (state === sIdle & launch) | state === sRead
  vop.io.in := rmem.read(raddr, ren)
  wmem.write(waddr, vop.io.out)

  // this prevents rmem to be removed
  when (io.wen) {
    rmem.write(io.waddr, io.wdata)
  }

  // this prevents wmem to be removed
  io.rdata := wmem.read(io.raddr, io.ren)

  // do not remove these registers
  dontTouch(launch)
  dontTouch(finish)
  dontTouch(length)
  dontTouch(cycle)
}

object Relu extends App {
  implicit val config = ReluConfig()
  chisel3.Driver.execute(args, () => new Relu)
}
