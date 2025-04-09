// Vertex Shader
struct VS_INPUT
{
    float4 position : POSITION; // Vertex position
    float4 color : COLOR;      // Vertex color
};

struct VS_OUTPUT
{
    float4 position : SV_POSITION; // Transformed position
    float4 color : COLOR;          // Output color
};

VS_OUTPUT main(VS_INPUT input)
{
    VS_OUTPUT output;
    
    // Simple pass-through: just return the position and color
    output.position = input.position;
    output.color = input.color;

    return output;
}