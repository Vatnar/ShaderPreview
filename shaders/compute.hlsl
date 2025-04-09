// Compute Shader
// Input Buffers: BufferA and BufferB, Output Buffer: Result

// Define the thread group size
[numthreads(256, 1, 1)] // One dimension, 256 threads per group
void main(uint3 dispatchThreadID : SV_DispatchThreadID)
{
    // Access the input buffers
    Buffer<float> BufferA : register(t0);
    Buffer<float> BufferB : register(t1);
    RWBuffer<float> Result : register(u0);

    // Calculate the index based on thread ID
    uint index = dispatchThreadID.x;

    // Perform element-wise addition and store the result
    Result[index] = BufferA[index] + BufferB[index];
}
