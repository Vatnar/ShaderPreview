// Pixel Shader
struct PS_INPUT
{
    float4 position : SV_POSITION; // Transformed position
    float4 color : COLOR;          // Input color
};

float4 main(PS_INPUT input) : SV_Target
{
    // Simply return the color passed in from the vertex shader
    return input.color;
}