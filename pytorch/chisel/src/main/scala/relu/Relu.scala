package relu

import chisel3._
import chisel3.util._

case class ReluConfig() {
  val xLen = 1
  val opDataWidth = 8
  val memDepth = 1024
  val memDataWidth = xLen * opDataWidth
  val memAddrWidth = log2Ceil(memDepth)
}

class Op(implicit config: ReluConfig) extends Module {
  val io = IO(new Bundle {
    val a = Input(UInt(config.opDataWidth.W))
    val y = Output(UInt(config.opDataWidth.W))
  })
  io.y := Mux(io.a.asSInt > 0.S, io.a, 0.U)
}

class Relu(implicit config: ReluConfig) extends Module {
  val io = IO(new Bundle{
    val wen = Input(Bool())
    val waddr = Input(UInt(config.memAddrWidth.W))
    val wdata = Input(UInt(config.memDataWidth.W))
    val ren = Input(Bool())
    val raddr = Input(UInt(config.memAddrWidth.W))
    val rdata = Output(UInt(config.memDataWidth.W))
  })
  val raddr = RegInit(0.U(config.memAddrWidth.W))
  val waddr = RegInit(0.U(config.memAddrWidth.W))
  val op = Module(new Op)
  val rmem = SyncReadMem(config.memDepth, UInt(config.memDataWidth.W))
  val wmem = SyncReadMem(config.memDepth, UInt(config.memDataWidth.W))
  op.io.a := rmem.read(raddr, true.B)
  wmem.write(waddr, op.io.y)
  when (io.wen) {
    rmem.write(io.waddr, io.wdata)
  }
  io.rdata := wmem.read(io.raddr, io.ren)
  dontTouch(raddr)
  dontTouch(waddr)
}

object Relu extends App {
  implicit val config = ReluConfig()
  chisel3.Driver.execute(args, () => new Relu)
}
