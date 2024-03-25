import os
import re
import time
import math

NAME = "DLA"

# Intermediary file
INTERMEDIARY_FILE_PATH = "../external_modules"
INTERMEDIARY_FILE_NAME = "dla_inter"

INTER_OPERATION_INDEX = 0
INTER_A_MATRIX_INDEX = 1
INTER_B_MATRIX_INDEX = 2
INTER_RESULT_INDEX = 3

# Memory banks
MEMORY_BANK_SIZE = 0x8000
NO_MEMORY_BANKS = 16

# Register map
BASE_ADDR = 0x1000
MEM_SIZE = 0x68
REG_WIDTH = 32

# Status register
STATUS_ADDR = 0x0 
BUF_DONE_OFFSET = 0x0
MAC_DONE_OFFSET = 0x1
PP_DONE_OFFSET = 0x2
DMA_IRQ_OFFSET = 0x3

# Control register
CTRL_ADDR = 0x4 
CPU_FE_OFFSET = 0x0
HP_RST_OFFSET = 0x4
SW_IRQ_OFFSET = 0x8

# Buffer control
BUF_CTRL = 0x8
CONV_MODE_OFFSET = 0x0
READ_A_VALID_OFFSET = 0x4
READ_B_VALID_OFFSET = 0x4 

# Mac control
MAC_CTRL = 0xC
SIMD_SELECT_OFFSET= 0x1
MAC_CLIP_OFFSET = 0x8

# PP control
PP_CTRL = 0x10
ACTIVE_MODE_OFFSET = 0x0
RELU_OFFSET = 0x2
MAX_OFFSET = 0x4
PP_SELECT_OFFSET = 0x6
POOL_MODE_OFFSET = 0x7
ROUNDING_OFFSET = 0x9
CTRL_VLD_OFFSET = 0xA
PP_CLIP_OFFSET = 0x10

# Buffer input
BUF_INPUT = 0x14
BUF_WIDTH_OFFSET = 0x0
BUF_HEIGHT_OFFSET = 0x9
BUF_CHANNELS_OFFSET = 0x12

# Buffer kernel 0
BUF_KERNEL_0 = 0x18
BUF_KERNEL_0_WIDTH_OFFSET = 0x0
BUF_KERNEL_0_HEIGHT_OFFSET = 0x4
BUF_KERNEL_0_S_CHANNELS_OFFSET = 0x8

# Buffer kernel 1
BUF_KERNEL_1 = 0x1C
BUF_KERNEL_1_NUM_OFFSET = 0x0

# Buffer padding
BUF_PAD = 0x20
BUF_PAD_TOP_OFFSET = 0x0
BUF_PAD_RIGHT_OFFSET = 0x4
BUF_PAD_BOTTOM_OFFSET = 0x8
BUF_PAD_LEFT_OFFSET = 0xC
BUF_PAD_VALUE_OFFSET = 0x10

# Buffer stride
BUF_STRIDE = 0x24 
BUF_STRIDE_X_OFFSET = 0x0
BUF_STRIDE_Y_OFFSET = 0x10

# PP input
PP_INPUT = 0x28
PP_INPUT_WIDTH_OFFSET = 0x0
PP_INPUT_HEIGHT_OFFSET = 0x10

# Buffer data bank
BUF_DATA_BANK = 0x2C
BUF_DATA_BANK_A_OFFSET = 0x0
BUF_DATA_BANK_B_OFFSET = 0x10

# Buffer data wait A
BUF_DATA_WAIT_A = 0x30
BUF_DATA_WAIT_A_OFFSET = 0x0

# Buffer data wait B
BUF_DATA_WAIT_B = 0x34
BUF_DATA_WAIT_B_OFFSET = 0x0

# Buffer pipe stall
BUF_PIPE_STALL_STALL_CYCLES = 0x38
BUF_PIPE_STALL_STALL_CYCLES_OFFSET = 0x0

# Power control
POWER_CTRL = 0x4C
POWER_CTRL_DOWN_0_OFFSET = 0x0
POWER_CTRL_DOWN_1_OFFSET = 0x1
POWER_CTRL_DOWN_2_OFFSET = 0x2
POWER_CTRL_ISO_OFFSET = 0x3

# Power status
POWER_STAT = 0x50
POWER_STAT_ACK_0_OFFSET = 0x0
POWER_STAT_ACK_1_OFFSET = 0x1
POWER_STAT_ACK_2_OFFSET = 0x2

# DMA control
DMA_CTRL = 0x44
DMA_CTRL_READ_EVENT_OFFSET = 0x0
DMA_CTRL_WRITE_EVENT_OFFSET = 0x0

# DMA padding
DMA_PAD_CONFIG = 0x48
DMA_PAD_CONFIG_OFFSET = 0x0

# MAC_SAT_MAX
MAC_SAT_MAX = 0x54
MAC_SAT_MAX_OFFSET = 0x0

# MAC_SAT_MIN
MAC_SAT_MIN = 0x58
MAC_SAT_MIN_OFFSET = 0x0

# PP_AXI_READ
PP_AXI_READ = 0x60
PP_AXI_READ_ADDRESS_OFFSET = 0x00

# Handshake
HANDSHAKE = 0x64
HANDSHAKE_BUFFER_VALID_OFFSET = 0x0
HANDSHAKE_MAC_VALID_OFFSET = 0x1
HANDSHAKE_POOL_VALID_OFFSET = 0x2
HANDSHAKE_ACTIVE_VALID_OFFSET = 0x3
HANDSHAKE_BUFFER_ENABLE_OFFSET = 0x4
HANDSHAKE_MAC_ENABLE_OFFSET =  0x5
HANDSHAKE_ACTIVE_ENABLE_OFFSET = 0x6
HANDSHAKE_POOL_ENABLE_OFFSET = 0x7
HANDSHAKE_BIAS_ENABLE_OFFSET = 0x8
HANDSHAKE_BYPASS_ENABLE_OFFSET = 0x9

class DLAExternal:
    def __init__(self):
        self.inter_file_path = os.path.join(INTERMEDIARY_FILE_PATH, INTERMEDIARY_FILE_NAME)
        self.clear()

    def update_file(self, lines):
        with open(self.inter_file_path, "w") as fp:
            for line in lines:
                fp.write("%s\n" % line)
        #print('Wrote to file:', lines)

    def write_conv2d(self, A, B):
        lines = ["\n"for _ in range(0,4)]
        lines[0] = "op:conv2d"
        lines[1] = "A:" + str(A)
        lines[2] = "B:" + str(B)
        lines[3] = "res:"
        self.update_file(lines)
        return

    def write_relu(self, A):
        lines = ["\n"for _ in range(0,4)]
        lines[0] = "op:conv2d"
        lines[1] = "A:" + str(A)
        lines[2] = ""
        lines[3] = "res:"
        self.update_file(lines)
        return

    def write_bias(self, A, b):
        return

    def write_matmul(self, A, B):
        lines = ["\n"for x in range(0,4)]
        lines[0] = "op:matmul"
        lines[1] = "A:" + str(A)
        lines[2] = "B:" + str(B)
        lines[3] = "res:"
        self.update_file(lines)
        return

    def read_lines(self):
        inter_file = open(self.inter_file_path, 'r')
        inter_file_lines = inter_file.readlines()
        inter_file.close()
        return inter_file_lines

    def read_result(self):
        while not self.has_result():
            pass
        lines = self.read_lines()
        result_line = lines[INTER_RESULT_INDEX].strip()
        result_line = result_line.split(":")[1]
        mat = self.parse_string_to_matrix(result_line)
        return mat

    def has_result(self):
        inter_file_lines = self.read_lines()
        if len(inter_file_lines) >= INTER_RESULT_INDEX + 1:
            result_line = inter_file_lines[INTER_RESULT_INDEX].strip()
            return result_line.startswith("res:") and len(result_line) > len("res:")
        return False

    def clear(self):
        lines = ["\n"for _ in range(0,4)]
        lines[0] = "op:clear"
        lines[1] = ""
        lines[2] = ""
        lines[3] = "res:"
        self.update_file(lines)

    def parse_string_to_matrix(self, s):
        s = s.replace("[[", "[")
        s = s.replace("]]", "]")
        pattern = r"\[(.*?)\]"
        rows = re.findall(pattern, s)
        mat = []
        for row in rows:
            numbers = row.split(",")
            r = []
            for n in numbers:
                r.append(float(n))
            mat.append(r)
        return mat


class MemoryBank:
    def __init__(self, size):
        self.size = size

class DLA:
    def __init__(self):
        self.mem = [0x00] * MEM_SIZE # Memory initalizaed
        self.name = "DLA"
        self.dla_external = DLAExternal()

        # Initialize memory banks
        self.banks = [MemoryBank(MEMORY_BANK_SIZE) for x in range(0, NO_MEMORY_BANKS)]

    def add_padding(self, A, padding=(1,1)):
        h_in = len(A)
        w_in = len(A[0])

        # Prepare padded output matrix
        out = [[0 for _ in range(w_in + padding[1])] for _ in range(h_in + padding[0])]

        # Insert A to padded matrix
        for i in range(h_in):
            for j in range(w_in):
                out[i + padding[0]][j + padding[1]] = A[i][j]

        return out

    def conv2d_check_parameters(self, A, kernel, padding, dilation, stride):
        h_in = len(A)
        w_in = len(A[0])
        h_kernel = len(kernel)
        w_kernel = len(kernel[0])
        h_out = math.floor((h_in + 2*padding[0] - dilation[0] * (h_kernel - 1) -1) / stride[0] +1)
        w_out = math.floor((w_in + 2*padding[1] - dilation[1] * (w_kernel - 1) -1) / stride[1] +1)

        h_middle_zero = math.floor((h_kernel / 2 % 1) * 2) # If 0 no middle, if 1 middle
        w_middle_zero = math.floor((w_kernel / 2 % 1) * 2) # If 0 no middle, if 1 middle

        return A, kernel, h_out, w_out, h_middle_zero, w_middle_zero


    # Perform conv2d to value written
    def conv2d_native(self, mat_in, kernel, padding=(0,0), dilation=(1,1), stride=(1,1)):

        mat_in, kernel, h_out, w_out, h_middle_zero, w_middle_zero = self.conv2d_check_parameters(mat_in, kernel, padding, dilation, stride)

        h_kernel = len(kernel)
        w_kernel = len(kernel[0])

        h_kernel_max_offset = h_kernel // 2 # Kernel height max offset
        w_kernel_max_offset = w_kernel // 2 # Kernel width max offset

        print("h_kernel_max_offset:", h_kernel_max_offset)
        print("w_kernel_max_offset:", w_kernel_max_offset)

        # Build array for output
        print("H_out:", h_out)
        print("W_out:", w_out)
        mat_out = [[0 for _ in range(w_out) ] for _ in range(h_out) ] # np.zeros(w_out, h_out)

        if w_middle_zero:
            center_x_0 = h_kernel_max_offset * dilation[0]
        else:
            center_x_0 = h_kernel_max_offset * dilation[0] -1

        if h_middle_zero:
            center_y_0 = w_kernel_max_offset * dilation[1]
        else:
            center_y_0 = w_kernel_max_offset * dilation[1] - 1

        print("h_middle_zero:", h_middle_zero)
        print("w_middle_zero:", w_middle_zero)
        print("center_x_0:", center_x_0, "center_y_0:", center_y_0)

        self.print_matrix(mat_in, "mat_in:")
        for j in range(h_out):
            if h_middle_zero:
                center_y = center_y_0 + j * stride[1]
                range_y = [center_y + k * dilation[1] for k in range(-h_kernel_max_offset, h_kernel_max_offset + 1)]
            else:
                center_y = (center_y_0) + j * stride[1] # Calculate from top left of center
                range_y = [center_y + k * dilation[1] for k in range(0, h_kernel_max_offset + 1)]

            for i in range(w_out):
                if w_middle_zero:
                    center_x = center_x_0 + i * stride[0]
                    range_x = [center_x + k * dilation[0] for k in range(-w_kernel_max_offset, w_kernel_max_offset + 1)]
                else:
                    center_x = (center_x_0) + i * stride[0]
                    range_x = [center_x + k * dilation[0] for k in range(0, w_kernel_max_offset + 1)]

                # Constuct a sub matrix
                mat_sub = [[0 for _ in range_x ] for _ in range_y ] # np.zeros(w_out, h_out)
                print("center_x:", center_x, "center_y:", center_y)
                print("range_x:", range_x, "range_y:", range_y)

                for mat_x in range(len(range_x)):
                    for mat_y in range(len(range_y)):
                        #print("mat_x:", mat_x, "mat_y:", mat_y)
                        mat_sub[mat_y][mat_x] = mat_in[range_y[mat_y]][range_x[mat_x]]

                self.print_matrix(mat_sub, "mat_sub:")
                self.print_matrix(kernel, "kernel:")
                mat_out[j][i] = self.mat_sum(self.matmul_element_wise(mat_sub, kernel))
                print("New cell value:", mat_out[j][i])
                self.print_matrix(mat_out, "mat_out_not_ready:")

        self.print_matrix(mat_out, "mat_out:")
        return mat_out

    def conv2d_external(self, A, B):
        self.dla_external.write_conv2d(A,B)
        C = self.dla_external.read_result()
        self.dla_external.clear()
        return C

    def print_matrix(self, A, name=""):
        print(name)
        for x in range(len(A)):
            row = ""
            for y in range(len(A[0])):
               row = row + str(A[x][y]) + " "
            print(row)


    # ReLU
    def relu_native(self, x):
        """Perform ReLU for input,
        relu is defined as (x + |x|) / 2, when x > 0 else 0
        """
        if x > 0:
            return (x + abs(x))/2
        else:
            return 0

    # Bias
    def bias_native(self, x, b):
        return x + b

    def matmul_element_wise(self, A, B):
        assert len(A) == len(B) and len(A[0]) == len(B[0])
        C = [[0 for _ in range(len(A[0])) ] for _ in range(len(A)) ] # np.zeros(w_out, h_out)
        for x in range(len(A)):
            for y in range(len(A[0])):
                C[x][y] = A[x][y] * B[x][y]
        return C


    def mat_sum(self, A):
        """Sum all cells in matrix together to produce single value
        """
        mat_sum = 0
        for x in range(len(A)):
            for y in range(len(A[0])):
                mat_sum = mat_sum + A[x][y]
        return mat_sum

    def matmul_external(self, A, B):
        self.dla_external.write_matmul(A, B)
        C = self.dla_external.read_result()
        self.dla_external.clear()
        return C

    def matmul_native(self, A, B):
        """Perform matrix multiplication AxB=C

        Params:
        A: Matrix A in form of [[Int]]
        B: Matrix B in form of [[Int]]

        Returns:
        C: Matrix C in form of [[Int]]
        """
        C = []
        for i in range(0,len(A)):
            temp=[]
            for j in range(0,len(B[0])):
                elem = 0
                for k in range(0,len(A[0])):
                    elem += A[i][k]*B[k][j]
                temp.append(elem)
            C.append(temp)
        return C

    def set_register(self, register, offset, width, value):
        """Set value of specific registers offset

        Params:
        register: interger of modified register
        offset: integer representing bit offset to start modification from
        width: width of the area in memory modified starting from offset
        value: value to be written to register
        """
        start = offset
        end = offset + width - 1
        self.mem[register] = self.mem[register] & ~((2 ** (end-start)) << (end))
        self.mem[register] = self.mem[register] | value << (offset)

    def get_registers(self):
        """Get all registers"""
        return self.mem

    def print_register(self, register):
        """Print specific register in b format

        Params:
        register: integer representing register to print
        """
        print("{0:x}: {1}".format(register, bin(self.mem[register])))

    def print_registers(self):
        """Print all registers"""
        for (i, reg) in enumerate(self.get_registers()):
            print("{:x}: {:b}".format(i, reg))

    def get_memory_banks(self):
        return self.banks

#    Logic    #
# ----------- #
#     API     #

def write(request, dla):
    self.NoisyLog("Writing request: %s at 0x%x, value 0x%x" % (str(request.type), request.offset, request.value))
    dla.mem[request.offset] = request.value

def read(request, dla):
    self.NoisyLog("Reading request: %s at 0x%x, value 0x%x" % (str(request.type), request.offset, request.value))
    request.value = dla.mem[request.offset]

if __name__ == "__main__":
    print("Running as independent module")
    dla = DLA()
    dla.print_registers()
    dla.set_register(HANDSHAKE, 3, 4, 3)
    dla.print_register(HANDSHAKE)
    A = [[1,2,3], [4,5,6], [7,8,9]]
    B = [[0,1], [1,0]]
    C = dla.matmul_element_wise([[5,6],[8,9]], [[0,1],[1,0]])
    print("Mat mul AxB=", C)
    C = dla.conv2d_native(A,B)
    print("Native Result AxB=", C)
    A = [[1,2], [3,4], [5,6]]
    C = dla.conv2d_native(A,B)
    print("Native Result AxB=", C)
    A = [[1,2,3], [4,5,6]]
    C = dla.conv2d_native(A,B)
    print("Native Result AxB=", C)
    A = [[1,2,3,4,5], [6,7,8,9,10], [11,12,13,14,15], [16,17,18,19,20], [21,22,23,24,25]]
    B = [[0,1,], [1,1], [0,1]]
    C = dla.conv2d_native(A,B)
    print("Native Result AxB=", C)







else:
    if request.isInit:
        dla = DLA()
        self.NoisyLog("%s initialized" % NAME)
    elif request.isRead:
        read(request, dla)
    elif request.isWrite:
        write(request, dla)
    else:
        self.NoisyLog("Bad request: %s at 0x%x, value 0x%x" % (str(request.type), request.offset, request.value))
