package relu

import chisel3._

case class ReluConfig() {
  val dataWidth = 8
}

class Op(implicit config: ReluConfig) extends Module {
  val io = IO(new Bundle {
    val a = Input(UInt(config.dataWidth.W))
    val y = Output(UInt(config.dataWidth.W))
  })
  io.y := Mux(io.a.asSInt > 0.S, io.a, 0.U)
}

class Relu(implicit config: ReluConfig) extends Module {
  val io = IO(new Bundle {
    val a = Input(UInt(config.dataWidth.W))
    val y = Output(UInt(config.dataWidth.W))
  })
  val a = RegInit(0.U(config.dataWidth.W))
  val y = RegInit(0.U(config.dataWidth.W))
  val op = Module(new Op)
  a := io.a
  io.y := y
  op.io.a := a
  y := op.io.y
}

object Relu extends App {
  implicit val config = ReluConfig()
  chisel3.Driver.execute(args, () => new Relu)
}
