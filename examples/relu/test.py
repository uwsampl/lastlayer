import os
import sys
import argparse
from relu import relu

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--num-vec-words", type=int, required=True, help="number of vector words")
    parser.add_argument("--repeat", type=int, required=True, help="repeat n times")
    args = parser.parse_args()
    relu_dir =  os.path.dirname(os.path.realpath(__file__))
    sys.path.append(relu_dir)
    n = 1024
    dir = "relu_{}".format(args.num_vec_words)
    lib = "librelu_{}.so".format(args.num_vec_words)
    relu_lib = os.path.join(relu_dir, dir, lib)
    vlen = []
    cycles = []
    etime = []
    for i in range(args.repeat):
        c, e = relu(relu_lib, n, args.num_vec_words)
        print("repeat:{} cycles:{} time:{}".format(args.repeat, c, e))
        #     vlen.append(args.num_vec_words * 4)
        #     cycles.append(c)
        #     etime.append(e)
        # d = {'vlen': vlen, 'cycles': cycles, 'time': etime}
        # filename = "relu_{}.csv".format(args.num_vec_words)
        # d.to_csv(filename, index=False)