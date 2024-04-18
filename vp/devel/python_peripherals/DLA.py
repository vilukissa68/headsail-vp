import os
import re
import math
import random

NAME = "DLA"

# Memory banks
MEMORY_BANK_ADDR = 0x70000000
MEMORY_BANK_SIZE = 0x8000
NO_MEMORY_BANKS = 16

# Register map
DLA_ADDR = 0xFF700000
REG_BASE_ADDR = 0x1000
MEM_SIZE = 0x68
REG_WIDTH = 32

# Status register
STATUS_ADDR = 0x0 
BUF_DONE_OFFSET = 0
MAC_DONE_OFFSET = 1
PP_DONE_OFFSET = 2
DMA_IRQ_OFFSET = 3

# Control register
CTRL_ADDR = 0x4 
CPU_FE_OFFSET = 0
HP_RST_OFFSET = 4
SW_IRQ_OFFSET = 8

# Buffer control
BUF_CTRL = 0x8
CONV_MODE_OFFSET = 0
READ_A_VALID_OFFSET = 4
READ_B_VALID_OFFSET = 8

# Mac control
MAC_CTRL = 0xC
SIMD_SELECT_OFFSET= 1
MAC_CLIP_OFFSET = 8

# PP control
PP_CTRL = 0x10
ACTIVE_MODE_OFFSET = 0
RELU_OFFSET_UNUSED = 2
MAX_OFFSET_UNUSED = 4
PP_SELECT_OFFSET = 6
POOL_MODE_OFFSET_UNUSED = 7
ROUNDING_OFFSET = 9
CTRL_VLD_OFFSET_UNUSED = 10
PP_CLIP_OFFSET = 16

# Buffer input
BUF_INPUT = 0x14
BUF_WIDTH_OFFSET = 0
BUF_HEIGHT_OFFSET = 9
BUF_CHANNELS_OFFSET = 18


# Buffer kernel 0
BUF_KERNEL_0 = 0x18
BUF_KERNEL_0_WIDTH_OFFSET = 0
BUF_KERNEL_0_HEIGHT_OFFSET = 4
BUF_KERNEL_0_S_CHANNELS_OFFSET = 8

# Buffer kernel 1
BUF_KERNEL_1 = 0x1C
BUF_KERNEL_1_NUM_OFFSET = 0

# Buffer padding
BUF_PAD = 0x20
BUF_PAD_TOP_OFFSET = 0
BUF_PAD_RIGHT_OFFSET = 4
BUF_PAD_BOTTOM_OFFSET = 8
BUF_PAD_LEFT_OFFSET = 12
BUF_PAD_VALUE_OFFSET = 16

# Buffer stride
BUF_STRIDE = 0x24 
BUF_STRIDE_X_OFFSET = 0
BUF_STRIDE_Y_OFFSET = 16

# PP input
PP_INPUT = 0x28
PP_INPUT_WIDTH_OFFSET = 0
PP_INPUT_HEIGHT_OFFSET = 16

# Buffer data bank
BUF_DATA_BANK = 0x2C
BUF_DATA_BANK_A_OFFSET = 0
BUF_DATA_BANK_B_OFFSET = 16

# Buffer data wait A
BUF_DATA_WAIT_A = 0x30
BUF_DATA_WAIT_A_OFFSET = 0

# Buffer data wait B
BUF_DATA_WAIT_B = 0x34
BUF_DATA_WAIT_B_OFFSET = 0

# Buffer pipe stall
BUF_PIPE_STALL_STALL_CYCLES = 0x38
BUF_PIPE_STALL_STALL_CYCLES_OFFSET = 0

# Power control
POWER_CTRL = 0x4C
POWER_CTRL_DOWN_0_OFFSET = 0
POWER_CTRL_DOWN_1_OFFSET = 1
POWER_CTRL_DOWN_2_OFFSET = 2
POWER_CTRL_ISO_OFFSET = 3

# Power status
POWER_STAT = 0x50
POWER_STAT_ACK_0_OFFSET = 0
POWER_STAT_ACK_1_OFFSET = 1
POWER_STAT_ACK_2_OFFSET = 2

# DMA control
DMA_CTRL = 0x44
DMA_CTRL_READ_EVENT_OFFSET = 0
DMA_CTRL_WRITE_EVENT_OFFSET = 0

# DMA padding
DMA_PAD_CONFIG = 0x48
DMA_PAD_CONFIG_OFFSET = 0

# MAC_SAT_MAX
MAC_SAT_MAX = 0x54
MAC_SAT_MAX_OFFSET = 0

# MAC_SAT_MIN
MAC_SAT_MIN = 0x58
MAC_SAT_MIN_OFFSET = 0

# PP_AXI_READ
PP_AXI_READ = 0x60
PP_AXI_READ_ADDRESS_OFFSET = 0

# Handshake
HANDSHAKE = 0x64
HANDSHAKE_BUFFER_VALID_OFFSET = 0
HANDSHAKE_MAC_VALID_OFFSET = 1
HANDSHAKE_POOL_VALID_OFFSET = 2
HANDSHAKE_ACTIVE_VALID_OFFSET = 3
HANDSHAKE_BUFFER_ENABLE_OFFSET = 4
HANDSHAKE_MAC_ENABLE_OFFSET =  5
HANDSHAKE_ACTIVE_ENABLE_OFFSET = 6
HANDSHAKE_POOL_ENABLE_OFFSET = 7
HANDSHAKE_BIAS_ENABLE_OFFSET = 8
HANDSHAKE_BYPASS_ENABLE_OFFSET = 9

# Utilities
def reshape_to_cwh(data):
    """Takes tensor in [height, width, channel] format and reshapes it to [channel, width, height]"""
    in_channels = len(data[0][0])
    in_width = len(data[0])
    in_heigth = len(data)

    # Initialize CWH array
    output = [[[0 for _ in range(in_heigth)] for _ in range(in_width)] for _ in range(in_channels)]

    # Reshape
    for ch in range(in_channels):
        for w in range(in_width):
            for h in range(in_heigth):
                output[ch][w][h] = data[h][w][ch]
    return output

def flatten_tensor(data):
    """Expect tensor in data CWH format and return 1d array"""
    in_height = len(data[0][0])
    in_width = len(data[0])
    in_channels = len(data)
    output = []
    for ch in range(in_channels):
        for w in range(in_width):
            for h in range(in_height):
                output.append(data[ch][w][h])
    return output

def flat_to_CWH(data, channels, width, height):
    """Takes in 1d array of length C*W*H and reshapes it to tensor of format CWH"""
    print(len(data), channels, width, height)
    assert channels * width * height == len(data)
    output = [[[0 for _ in range(height)] for _ in range(width)] for _ in range(channels)]
    i = 0
    for ch in range(channels):
        for w in range(width):
            for h in range(height):
                output[ch][w][h] = data[i]
                i = i + 1
    return output

def separate_channels(data):
    """Reformats data so that each channels is it's own 2D array

    Params:
    data -- [[[Int]]] Data in format CWH

    Return:
    channel_matrices
    """
    num_channels, width, height = len(data), len(data[0]), len(data[0][0])
    channel_matrices = []
    for i in range(num_channels):
        channel_matrix = []
        for j in range(width):
            row = []
            for k in range(height):
                row.append(data[i][j][k])
            channel_matrix.append(row)
        channel_matrices.append(channel_matrix)
    return channel_matrices

def print_matrix(A, name=""):
    """Print matrix"""
    print(name)
    for x in range(len(A)):
        row = ""
        for y in range(len(A[0])):
            row = row + str(A[x][y]) + " "
        print(row)

def execute_for_all_elements(f, tensor, *args):
    """Execute function that takes a single matrix element for all element in a matrix.

    Params:
    f -- Function(Int, *args)->Int Function to be applied to matrix elements. Needs to return same element as it operates on.
    tensor -- ndarray(Int) Multidimensional array with number elements to apply function f to.
    *args -- *args Additional arguments needed by function f.

    Returns:
    tensor -- ndarray(Int) Tensor with function f applied to all it's elements
    """
    if isinstance(tensor, list):  # Check if tensor is a list
        return [execute_for_all_elements(f, x, *args) for x in tensor]
    else:  # Base case: tensor is not a list, apply f
        return f(tensor, *args)

def clip(value, clip, no_overflow=False):
    """Value to possibly clip is clipped to max of bit length set by clip
    params:
    value = value to clip
    clip =  amount of bits allowed
    return:
    tuple (a, b)
    a = value resulting from the clipping
    b = amount of owerflow due to clipping, 0 if no clipping
    """
    mask = sum(range(0,clip + 1))
    if value > mask:
        if no_overflow:
            return mask
        return (mask, value - mask)
    else:
        if no_overflow:
            value
        return (value, 0)

def rounding(value):
    """Round given value

    Params:
    value -- Float value to round

    Return:
    rounded -- Int Rounded value
    """
    return round(value)


# DLA
class MemoryBank:
    """Implements DLA memory bank"""
    def __init__(self, size):
        self.size = size
        self.clear_bank()
        self.idx = 0 # Data pointer

    def clear_bank(self):
        """Reset all values in the bank to 0."""
        self.mem = [0 for x in range(self.size)] # Initialize bank

    def is_bank_full(self):
        """Check if bank is full

        Returns:
        is_full -- Bool True if full. False if not full.
        """
        return self.size == self.idx

    def write(self, data):
        """Write data to bank. Starting from first free address.

        Params:
        data -- [Int] data to write to bank

        Returns:
        unwritten -- [Int] data that didn't fit to bank
        """
        for i in range(len(data)):
            if self.is_bank_full():
                # Memory has been filled
                return data[i:]
            byte = data[i]
            self.mem[self.idx] = byte
            self.idx = self.idx + 1 # Increment for pointer
        return [] # All the data fitted to the bank

    def read(self):
        """Read all the values written to the bank. Stop at first free address.

        Return:
        data -- [Int] all data in the bank
        """
        return self.mem[0:self.idx] # Read until write pointer, rest is padded

    def available_memory(self):
        """Get the amount of free space left in the bank.

        Return:
        free -- Int Free space in bank
        """
        return self.size - self.idx


class Dla:
    """Implements control flow and MMIO registers of DLA. This should be the top level component."""
    def __init__(self):
        self.mem = bytearray(MEM_SIZE) # Memory initalizaed
        self.mac = DlaMac()
        # Initialize memory banks
        self.banks = [MemoryBank(MEMORY_BANK_SIZE) for x in range(0, NO_MEMORY_BANKS)]

        # Initialize register to correct values
        # Start processing data in buffers
        self.process()

    def write(self, offset, value):
        """Writes value to register with given offset

        Params:
        offset -- Int memory address offset to write
        value -- Int value to write
        """
        self.mem[offset] = value

    def read(self, offset):
        """Read value of register with given offset

        Params:
        offset -- Int memory address offset to read

        Returns:
        value -- value of register in given offset
        """
        return self.mem[offset]

    def set_register(self, register, offset, width, value):
        """Set value of specific registers offset

        Params:
        register -- base address of 32-bit "register"
        offset -- integer representing bit offset to start modification from
        width -- width of the area in memory modified starting from offset
        value -- value to be written to register
        """

        start_reg = register - (register % 4)
        full_reg = self.mem[register:register+4]
        full_reg = full_reg[0] +  (full_reg[1] << 8)  + (full_reg[2] << 16) + (full_reg[3] << 24)
        zeroing_mask = ~(((width*width)-1) << offset)

        full_reg = full_reg & zeroing_mask # Zero region to be written
        full_reg = full_reg | (value << offset)

        self.mem[register] = full_reg & 0xFF
        self.mem[register + 1] = (full_reg >> 8) & 0xFF
        self.mem[register + 2] = (full_reg >> 16) & 0xFF
        self.mem[register + 3] = (full_reg >> 24) & 0xFF

    def get_register(self, register, offset, width):
        """Read value of specific bits from a register

        Params:
        register -- integer of register to read from
        offset -- integer representing bit offset to start reading from
        width -- width of the area in memory to read starting from offset

        Returns:
        value -- value read from the specified bytes in the register
        """
        full_reg = self.mem[register:register+4]
        full_reg = full_reg[0] +  (full_reg[1] << 8)  + (full_reg[2] << 16) + (full_reg[3] << 24)
        value = full_reg >> offset
        mask = sum(range(0,width+1))
        return value & mask

    def get_registers(self):
        """Get all registers

        Return:
        mem -- [Int] all registers values in DLA
        """
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
            #print("{:x}: {:b}".format(i, reg))
            print("{}: {}".format(hex(i), hex(reg)))
        return

    def get_memory_banks(self):
        """Get all memory banks

        Return:
        banks -- [MemoryBank] List of all the MemoryBank objects
        """
        return self.banks

    def handle_bank_write(self, request):
        """Writes data request to memory bank based on request.absolute address

        Params:
        request -- Renode request object
        """
        target_bank = (request.absolute - MEMORY_BANK_ADDR) // MEMORY_BANK_SIZE
        self.banks[target_bank].write([request.value])

    def set_weight_data(self, data):
        """Sets weight data to memory banks

        Params:
        data -- [Int] Weight data in CHW format

        Returns:
        unwritten -- [Int] All the data that didn't fit into memory banks. Empty if all fitted.
        """

        bank_idx = self.get_register(BUF_DATA_BANK, BUF_DATA_BANK_A_OFFSET, 4)
        i = 0
        while i < len(data):
            bank = self.banks[bank_idx]
            if bank.is_bank_full(): # Move to next bank
                bank_idx = bank_idx + 1
                # Check that next bank isn't for input data
                if bank_idx == self.get_register(BUF_DATA_BANK, BUF_DATA_BANK_B_OFFSET, 4) or bank_idx > MEMORY_BANK_SIZE:
                    break;
            else:
                if i+bank.available_memory() > len(data):
                    slice = data[i:]
                    i = len(data)
                else:
                    slice = data[i:i+bank.available_memory()]
                    i = i + bank.available_memory()
                bank.write(slice)
        if i != len(data):
            return data[i:]
        return []

    def set_input_data(self, data):
        """Sets input data to memory banks

        Params:
        data -- [Int] Input data in CHW format

        Returns:
        unwritten -- [Int] All the data that didn't fit into memory banks. Empty if all fitted.
        """
        bank_idx = self.get_register(BUF_DATA_BANK, BUF_DATA_BANK_B_OFFSET, 4)
        i = 0
        while i < len(data):
            bank = self.banks[bank_idx]
            if bank.is_bank_full(): # Move to next bank
                bank_idx = bank_idx + 1
                # Check that next bank isn't for weight data
                if bank_idx == self.get_register(BUF_DATA_BANK, BUF_DATA_BANK_A_OFFSET, 4) or bank_idx > MEMORY_BANK_SIZE:
                    break;
            else:
                if i+bank.available_memory() > len(data):
                    slice = data[i:]
                    i = len(data)
                else:
                    slice = data[i:i+bank.available_memory()]
                    i = i + bank.available_memory()
                bank.write(slice)
        if i != len(data):
            return data[i:]
        return []

    def get_weight_data(self):
       """Get all kernel/weight data from memory banks in CWH format

       Returns:
       channels -- Int Number of channels
       width -- Int Width of input
       height -- Int Height of input
       data -- [Int] List of all the weight values in CWH format
       """
       width = self.get_register(BUF_KERNEL_0, BUF_KERNEL_0_WIDTH_OFFSET, 4) + 1
       height = self.get_register(BUF_KERNEL_0, BUF_KERNEL_0_HEIGHT_OFFSET, 4) + 1
       channels = self.get_register(BUF_KERNEL_0, BUF_KERNEL_0_S_CHANNELS_OFFSET, 4) + 1
       bank_idx = self.get_register(BUF_DATA_BANK, BUF_DATA_BANK_A_OFFSET, 4)
       bank = self.banks[bank_idx]

       # Check if multiples banks are occupied
       data = bank.read()
       while bank.is_bank_full():
           bank_idx = bank_idx + 1
           bank = self.banks[bank_idx]

           # Check that next bank isn't for input data
           if bank_idx == self.get_register(BUF_DATA_BANK, BUF_DATA_BANK_B_OFFSET, 4) or bank_idx > MEMORY_BANK_SIZE:
               break;

           # If new bank is valid, read
           data = data + bank.read()

       return channels, width, height, data

    def get_input_data(self):
        """Get all input data from memory banks in CWH format

        Returns:
        channels -- Int Number of channels
        width -- Int Width of input
        Height -- Int Height of input
        data -- [Int] List of all the input values in CWH format
        """
        width = self.get_register(BUF_INPUT, BUF_WIDTH_OFFSET, 9) + 1
        height = self.get_register(BUF_INPUT, BUF_HEIGHT_OFFSET, 9) + 1
        channels = self.get_register(BUF_INPUT, BUF_CHANNELS_OFFSET, 12) + 1
        bank_idx = self.get_register(BUF_DATA_BANK, BUF_DATA_BANK_B_OFFSET, 4)
        bank = self.banks[bank_idx]

        # Check if multiple banks are occupied
        data = bank.read()
        while bank.is_bank_full():
            bank_idx = bank_idx + 1
            bank = self.banks[bank_idx]

            # check that next bank isn't for weight data
            if bank_idx == self.get_register(BUF_DATA_BANK, BUF_DATA_BANK_A_OFFSET, 4) or bank_idx > MEMORY_BANK_SIZE:
                break;

            # If new bank is valid, read
            data = data + bank.read()

        return channels, width, height, data

    # TODO: Finish this when external memory is figured out
    def get_bias(self):
       """Get bias values from external memory"""
       bias_address = self.get_register(PP_AXI_READ, PP_AXI_READ_ADDRESS_OFFSET, 32)
       #data = self.external[bias_address]
       return 0

    def get_conv_params(self):
       """Get parameters for conv2d

       Returns:
       padding -- Tuple (Int, Int) sets padding in (x,y) direction
       dilation -- Tuple (Int, Int) sets dilation in (x,y) direction
       stride -- Tuple (Int, Int) sets stride in (x,y) direction
       """
       pad_x = self.get_register(BUF_PAD, BUF_PAD_LEFT_OFFSET, 4)
       pad_y = self.get_register(BUF_PAD, BUF_PAD_TOP_OFFSET, 4)
       stride_x = self.get_register(BUF_STRIDE, BUF_STRIDE_X_OFFSET, 4) + 1
       stride_y = self.get_register(BUF_STRIDE, BUF_STRIDE_Y_OFFSET, 4) + 1
       dilation_x = 1 # NOTE: Headsail's DLA doesn't support other dilations
       dilation_y = 1

       return (pad_x, pad_y), (dilation_x, dilation_y), (stride_x, stride_y)

    def handle_handshake(self):
        """Resets handshake registers correctly after succesful calculation"""
        if self.get_register(HANDSHAKE, HANDSHAKE_BUFFER_VALID_OFFSET, 1) == 1 and self.get_register(HANDSHAKE, HANDSHAKE_BUFFER_ENABLE_OFFSET, 1) == 0 :
           self.set_register(HANDSHAKE, HANDSHAKE_BUFFER_VALID_OFFSET, 1, 0)
           self.set_register(STATUS_ADDR, BUF_DONE_OFFSET, 1, 0)
        if self.get_register(HANDSHAKE, HANDSHAKE_MAC_VALID_OFFSET, 1) == 1 and self.get_register(HANDSHAKE, HANDSHAKE_MAC_ENABLE_OFFSET, 1) == 0 :
           self.set_register(HANDSHAKE, HANDSHAKE_MAC_VALID_OFFSET, 1, 0)
           self.set_register(STATUS_ADDR, MAC_DONE_OFFSET, 1, 0)
        if self.get_register(HANDSHAKE, HANDSHAKE_ACTIVE_VALID_OFFSET, 1) == 1 and self.get_register(HANDSHAKE, HANDSHAKE_BYPASS_ENABLE_OFFSET, 1) == 0 :
           self.set_register(HANDSHAKE, HANDSHAKE_ACTIVE_VALID_OFFSET, 1, 0)
           self.set_register(STATUS_ADDR, PP_DONE_OFFSET, 1, 0)

    def round(self, values):
        """Round values if register is set"""
        if self.get_register(PP_CTRL, ROUNDING_OFFSET, 1) == 1:
            return execute_for_all_elements(rounding, values)
        return values

    def pp_clip(self, values):
        """Clip pp values if register is set"""
        clip_amount = self.get_register(PP_CTRL, PP_CLIP_OFFSET, 5)
        if clip_amount > 0:
            return execute_for_all_elements(clip, values, clip_amount, True)
        return values

    def mac_clip(self, values):
        """Clip mac values if register is set"""
        clip_amount = self.get_register(MAC_CTRL, MAC_CLIP_OFFSET, 5)
        if clip_amount > 0:
            return execute_for_all_elements(clip, values, clip_amount, True)
        return values

    def process(self):
        """Runs next tick of the DLA state"""
        # Check if data is ready
        if not self.get_register(BUF_CTRL, READ_A_VALID_OFFSET, 1) or not self.get_register(BUF_CTRL, READ_B_VALID_OFFSET, 1):
            return

        # Load data from memory banks and reshape
        input_ch, input_w, input_h, input_data = self.get_input_data()
        input_data = flat_to_CWH(input_data, input_ch, input_w, input_h)

        kernel_ch, kernel_w, kernel_h, kernel_data = self.get_weight_data()
        kernel_data = flat_to_CWH(kernel_data, kernel_ch, kernel_w, kernel_h)

        # Convonlution
        # Read convolution paramters
        padding, dilation, stride = self.get_conv_params()
        # Execute op in MAC array
        # Once for each channel
        # TODO: This might be not correct, make sure S_CHANNELS work like this
        res = []
        for channel in range(input_ch):
            for k_channel in range(kernel_ch):
                res.append(self.mac.conv2d_native(input_data[channel], kernel_data[k_channel], padding, dilation, stride))

        # Clip results
        res = dla.mac_clip(res)

        print("Results:")
        for r in res:
            print_matrix(r)

        # TODO: Post process
        # Check if PP is used
        if self.get_register(PP_CTRL, PP_SELECT_OFFSET, 1):
            # TODO: Bias
            if True:
                #bias_amount = self.get_bias()
                bias_amount = 1
                res = execute_for_all_elements(self.mac.bias_native, res, bias_amount)
                print("bias:")
                for r in res:
                    print_matrix(r)

            # ReLU (active low)
            if self.get_register(PP_CTRL, ACTIVE_MODE_OFFSET, 2) == 0:
                res = execute_for_all_elements(self.mac.relu_native, res)
                res = dla.pp_clip(res)
                res = dla.round(res)
                print("ReLU:")
                for r in res:
                    print_matrix(r)

            # TODO: Pool

        # TODO: Write result somewhere

        # Set Done status
        self.set_register(STATUS_ADDR, BUF_DONE_OFFSET, 1, 0x1)
        self.set_register(STATUS_ADDR, MAC_DONE_OFFSET, 1, 0x1)
        self.set_register(STATUS_ADDR, PP_DONE_OFFSET, 1, 0x1)

        # TODO: Wait for validation

        # After validation set done bits to 0
        self.set_register(STATUS_ADDR, BUF_DONE_OFFSET, 1, 0x0)
        self.set_register(STATUS_ADDR, MAC_DONE_OFFSET, 1, 0x0)
        self.set_register(STATUS_ADDR, PP_DONE_OFFSET, 1, 0x0)


class DlaMac:
    """Implement DLA's MAC array operations Conv2d, Bias and ReLU"""
    def __init__(self):
        self.name = "DLA MAC"

    def add_padding(self, A, padding=(1,1)):
        """Pads input matrix

        Params:
        A -- Matrix in form [[Int]] to pad
        padding -- Tuple (Int, Int) sets padding in (x,y) direction

        Returns:
        out -- Padded matrix A
        """
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
        """Calculates outputs of convolution matrix based on preferred inputs

        Params:
        A -- Matrix in form [[Int]] to be convolved
        kernel -- Matrix in form of [[Int]] to use ase convolution kernel
        padding -- Tuple (Int, Int) sets padding in (x,y) direction
        dilation -- Tuple (Int, Int) sets dilation in (x,y) direction
        stride -- Tuple (Int, Int) sets stride in (x,y) direction

        Returns:
        A -- Matrix in form [[Int]] to be convolved
        kernel -- Matrix in form of [[Int]] to use ase convolution kernel
        h_out -- Height of the conv2d result matrix
        w_out -- Width of the conv2d result matrix
        h_middle_zero -- Bool signifying if conv2d has uneven height
        w_middle_zero -- Bool signifying if conv2d has uneven width
        """

        h_in = len(A)
        w_in = len(A[0])
        h_kernel = len(kernel)
        w_kernel = len(kernel[0])
        h_out = math.floor((h_in + 2*padding[0] - dilation[0] * (h_kernel - 1) -1) / stride[0] +1)
        w_out = math.floor((w_in + 2*padding[1] - dilation[1] * (w_kernel - 1) -1) / stride[1] +1)

        h_middle_zero = h_kernel % 2 == 1
        w_middle_zero = w_kernel % 2 == 1

        return A, kernel, int(h_out), int(w_out), h_middle_zero, w_middle_zero


    # Perform conv2d to value written
    def conv2d_native(self, mat_in, kernel, padding=(0,0), dilation=(1,1), stride=(1,1)):
        """2D convolution

        Params:
        mat_in -- Matrix in form [[Int]] to be convolved
        kernel -- Matrix in form of [[Int]] to use ase convolution kernel
        padding -- Tuple (Int, Int) sets padding in (x,y) direction
        dilation -- Tuple (Int, Int) sets dilation in (x,y) direction
        stride -- Tuple (Int, Int) sets stride in (x,y) direction

        Returns:
        mat_out -- Result of convolution operation in form of [[Int]]
        """

        mat_in, kernel, h_out, w_out, h_middle_zero, w_middle_zero = self.conv2d_check_parameters(mat_in, kernel, padding, dilation, stride)

        h_kernel = len(kernel)
        w_kernel = len(kernel[0])

        h_kernel_max_offset = h_kernel // 2 # Kernel height max offset
        w_kernel_max_offset = w_kernel // 2 # Kernel width max offset

        # Build array for output
        mat_out = [[0 for _ in range(w_out) ] for _ in range(h_out) ] # np.zeros(w_out, h_out)

        if w_middle_zero:
            center_x_0 = h_kernel_max_offset * dilation[0]
        else:
            center_x_0 = h_kernel_max_offset * dilation[0] -1

        if h_middle_zero:
            center_y_0 = w_kernel_max_offset * dilation[1]
        else:
            center_y_0 = w_kernel_max_offset * dilation[1] - 1

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

                for mat_x in range(len(range_x)):
                    for mat_y in range(len(range_y)):
                        mat_sub[mat_y][mat_x] = mat_in[range_y[mat_y]][range_x[mat_x]]

                mat_out[j][i] = self.mat_sum(self.matmul_element_wise(mat_sub, kernel))

        return mat_out


    # ReLU
    def relu_native(self, x):
        """Perform ReLU for input,

        Params:
        x -- constant input Int

        Returns:
        ReLU(x) -- always positive integer with ReLU applied
        """
        if x > 0:
            return x
        else:
            return 0

    # Bias
    def bias_native(self, x, b):
        """Performs bias operation for input
        Params:
        x -- constant input Int
        b -- constant input bias Int

        Returns:
        sum -- x + b
        """
        return x + b

    def matmul_element_wise(self, A, B):
        """Multiply elements between matrices A and B

        Params:
        A -- Matrix A in form of [[Int]]
        B -- Matrix B in form of [[Int]]

        Returns:
        C -- Matrix C in form of [[Int]]
        """
        assert len(A) == len(B) and len(A[0]) == len(B[0])
        C = [[0 for _ in range(len(A[0])) ] for _ in range(len(A)) ] # np.zeros(w_out, h_out)
        for x in range(len(A)):
            for y in range(len(A[0])):
                C[x][y] = A[x][y] * B[x][y]
        return C

    def mat_sum(self, A):
        """Sum of all matrix cells

        Params:
        A -- Matrix A in form of [[Int]]

        Returns:
        mat_sum -- sum of matrix A elements
        """
        mat_sum = 0
        for x in range(len(A)):
            for y in range(len(A[0])):
                mat_sum = mat_sum + A[x][y]
        return mat_sum

    def matmul_native(self, A, B):
        """Perform matrix multiplication AxB=C

        Params:
        A -- Matrix A in form of [[Int]]
        B -- Matrix B in form of [[Int]]

        Returns:
        C -- Matrix C in form of [[Int]]
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


#    Logic    #
# ----------- #
#     API     #

def write(request, dla):
    self.NoisyLog("Absolute: 0x%x  Writing request offset: %s at 0x%x, value 0x%x" % (request.absolute, str(request.type), request.offset, request.value))
    print("Absolute: 0x%x  Writing request offset: %s at 0x%x, value 0x%x" % (request.absolute, str(request.type), request.offset, request.value))
    if int(request.absolute) >= DLA_ADDR:
        dla.set_register(request.offset, 0, 32, request.value)
    else:
        # TODO: implement bank writing
        dla.handle_bank_write(request)
    dla.process()

def read(request, dla):
    self.NoisyLog("Reading request: %s at 0x%x, value 0x%x" % (str(request.type), request.absolute, request.value))
    print("Absolute: 0x%x  Reading request offset: %s at 0x%x, value 0x%x" % (request.absolute, str(request.type), request.offset, request.value))
    if int(request.absolute) >= DLA_ADDR:
        request.value = dla.get_register(request.offset, 0, 32)
    else:
        request.value = 0 # TODO: Add bank reading here


if __name__ == "__main__":
    print("Running as independent module")
    dla = Dla()

    A_ch, A_h, A_w = 3, 5, 5
    B_ch, B_h, B_w = 1, 3, 3

    # Set input size
    dla.set_register(BUF_INPUT, BUF_CHANNELS_OFFSET, 12, A_ch - 1)
    dla.set_register(BUF_INPUT, BUF_WIDTH_OFFSET, 9, A_h - 1)
    dla.set_register(BUF_INPUT, BUF_HEIGHT_OFFSET, 9, A_w - 1)

    # Set weight size
    dla.set_register(BUF_KERNEL_0, BUF_KERNEL_0_S_CHANNELS_OFFSET, 12, B_ch - 1)
    dla.set_register(BUF_KERNEL_0, BUF_KERNEL_0_WIDTH_OFFSET, 4, B_h - 1)
    dla.set_register(BUF_KERNEL_0, BUF_KERNEL_0_HEIGHT_OFFSET, 4, B_w - 1)

    # Set banks for input and weight data
    dla.set_register(BUF_DATA_BANK, BUF_DATA_BANK_A_OFFSET, 4, 0)
    dla.set_register(BUF_DATA_BANK, BUF_DATA_BANK_B_OFFSET, 4, 8)

    # Generate 256 x 256 image
    A = [[[random.randint(-1,1) for _ in range(A_ch) ] for _ in range(A_h)] for _ in range(A_w)]
    B = [[[random.randint(-1, 1) for _ in range(B_ch) ] for _ in range(B_h)] for _ in range(B_w)]
    A = reshape_to_cwh(A) # HWC to CWH
    B = reshape_to_cwh(B)

    A = separate_channels(A) # CHW to 2D
    B = separate_channels(B)
    C = dla.mac.conv2d_native(A[0], B[0])
    print_matrix(C, "C:")

    # Write input data to data bank
    A = flatten_tensor(A)
    dla.set_input_data(A)

    # Write weight data to data bank
    B = flatten_tensor(B)
    dla.set_weight_data(B)

    # Initialization ready
    dla.set_register(HANDSHAKE, HANDSHAKE_BUFFER_ENABLE_OFFSET, 1, 0x1) # Data buffer
    dla.set_register(HANDSHAKE, HANDSHAKE_MAC_ENABLE_OFFSET, 1, 0x1) # DLA array
    dla.set_register(HANDSHAKE, HANDSHAKE_BYPASS_ENABLE_OFFSET, 1, 0x1) # Post processor

    # Enable PP
    dla.set_register(PP_CTRL, PP_SELECT_OFFSET, 1, 1)
    dla.set_register(PP_CTRL, ACTIVE_MODE_OFFSET, 2, 0)

    # Input data is ready
    dla.set_register(BUF_CTRL, READ_A_VALID_OFFSET, 1, 0x1) # All weight data ready
    dla.set_register(BUF_CTRL, READ_B_VALID_OFFSET, 1, 0x1) # All input data ready

    dla.process()

else:
    if request.isInit:
        dla = Dla()
        print("%s initialized" % NAME)
        self.NoisyLog("%s initialized" % NAME)
    elif request.isRead:
        print("isRead")
        read(request, dla)
    elif request.isWrite:
        write(request, dla)
    else:
        self.NoisyLog("Bad request: %s at 0x%x, value 0x%x" % (str(request.type), request.offset, request.value))
        print("Bad request: %s at 0x%x, value 0x%x" % (str(request.type), request.offset, request.value))
