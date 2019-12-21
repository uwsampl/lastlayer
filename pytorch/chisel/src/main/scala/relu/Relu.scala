package relu

import chisel3._

class Relu extends Module {
  val io = IO(new Bundle {
    val value = Input(UInt(16.W))
  })
}

object Relu extends App {
  chisel3.Driver.execute(args, () => new Relu)
}
