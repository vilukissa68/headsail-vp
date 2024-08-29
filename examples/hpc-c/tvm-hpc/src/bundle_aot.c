/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

/*!
 * \brief Implementation of TVMPlatform functions in tvm/runtime/crt/platform.h
 */

#include <dlpack.h>
#include <tvm/runtime/crt/error_codes.h>
#include <tvm/runtime/crt/stack_allocator.h>
#include <tvm/runtime/crt/aot_executor.h>
#include "stdarg.h"
#include "stdlib.h"
#include "stdio.h"

// AOT memory array
#define WORKSPACE_SIZE 21312
static uint8_t g_aot_memory[WORKSPACE_SIZE];
//extern tvm_model_t tvmgen_default_network;
tvm_workspace_t app_workspace;

// Called when an internal error occurs and execution cannot continue.
void TVMPlatformAbort(tvm_crt_error_t error) {
    exit(-1);
}

void TVMLogf(const char* msg, ...) {}

// Called by the microTVM RPC server to implement TVMLogf.
size_t TVMPlatformFormatMessage(char* out_buf, size_t out_buf_size_bytes, const char* fmt,
                                va_list args) {
  return vsnprintf(out_buf, out_buf_size_bytes, fmt, args);
}

// Allocate memory for use by TVM.
tvm_crt_error_t TVMPlatformMemoryAllocate(size_t num_bytes, DLDevice dev, void** out_ptr) {
  if (num_bytes == 0) {
    num_bytes = sizeof(int);
  }
  *out_ptr = malloc(num_bytes);
  return (*out_ptr == NULL) ? kTvmErrorPlatformNoMemory : kTvmErrorNoError;
}

// Free memory used by TVM.
tvm_crt_error_t TVMPlatformMemoryFree(void* ptr, DLDevice dev) {
  free(ptr);
  return kTvmErrorNoError;
}

unsigned long g_utvm_start_time_micros;
int g_utvm_timer_running = 0;

// Start a device timer.
tvm_crt_error_t TVMPlatformTimerStart() {
    return kTvmErrorNoError;
}

// Stop the running device timer and get the elapsed time (in microseconds).
tvm_crt_error_t TVMPlatformTimerStop(double* elapsed_time_seconds) {
    return kTvmErrorNoError;
}

// Fill a buffer with random data.
tvm_crt_error_t TVMPlatformGenerateRandom(uint8_t* buffer, size_t num_bytes) {
  return kTvmErrorNoError;
}

//void TVMInitialize() { StackMemoryManager_Init(&app_workspace, g_aot_memory, WORKSPACE_SIZE); }

void TVMExecute(void* input_data, void* output_data) {
  int ret_val = tvmgen_default_run(input_data, output_data);
  if (ret_val != 0) {
    TVMPlatformAbort(kTvmErrorPlatformCheckFailure);
  }
}
