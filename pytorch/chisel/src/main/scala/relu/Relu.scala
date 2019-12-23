package relu

import chisel3._
import chisel3.util._

case class ReluConfig() {
  val xLen = 1
  val opDataWidth = 8
  val memDepth = 1024
  val memDataWidth = xLen * opDataWidth
  val memAddrWidth = log2Ceil(memDepth)
  val counterWidth = 16
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
  val vop = Module(new VecOp)
  val rmem = SyncReadMem(config.memDepth, UInt(config.memDataWidth.W))
  val wmem = SyncReadMem(config.memDepth, UInt(config.memDataWidth.W))
  val active = RegInit(false.B)
  val idle = RegInit(false.B)
  val cycle = RegInit(0.U(config.counterWidth.W))
  val numop = RegInit(0.U(config.memAddrWidth.W))
  val length = RegInit(0.U(config.memAddrWidth.W))
  val raddr = RegInit(0.U(config.memAddrWidth.W))
  val waddr = RegInit(0.U(config.memAddrWidth.W))

  when (active && !idle) {
    raddr := raddr + 1.U
    waddr := waddr + 1.U
    cycle := cycle + 1.U
    numop := numop + config.xLen.U
  }

  when (numop === length) {
    idle := true.B
    active := false.B
  }

  vop.io.in := rmem.read(raddr, active)
  wmem.write(waddr, vop.io.out)

  // this prevents rmem to be removed
  when (io.wen) {
    rmem.write(io.waddr, io.wdata)
  }

  // this prevents wmem to be removed
  io.rdata := wmem.read(io.raddr, io.ren)

  // do not remove these registers
  dontTouch(active)
  dontTouch(length)
  dontTouch(cycle)
}

object Relu extends App {
  implicit val config = ReluConfig()
  chisel3.Driver.execute(args, () => new Relu)
}
