import TriangleShadersCode from "./triangle-shaders.wgsl?raw";

async function main() {
  const adapter = await navigator.gpu.requestAdapter();
  const device = await adapter?.requestDevice();
  if (!device) {
    console.error("WebGPU is not supported on this device");
    return;
  }

  console.log("WebGPU is supported on this device");
  console.group("WebGPU Device Information");
  console.log("Adapter: ", adapter);
  console.log("Device: ", device);
  console.groupEnd();

  const canvas = document.getElementById("canvas") as HTMLCanvasElement;
  const context = canvas.getContext("webgpu")!;
  const presentationFormat = navigator.gpu.getPreferredCanvasFormat();
  context.configure({
    device,
    format: presentationFormat,
  });

  const module = device.createShaderModule({
    label: "our hardcoded red triangle shaders",
    code: TriangleShadersCode,
  });
  const pipeline = device.createRenderPipeline({
    label: "our hardcoded red triangle pipeline",
    layout: "auto",
    vertex: {
      module,
      entryPoint: "vs",
    },
    fragment: {
      module,
      entryPoint: "fs",
      targets: [
        {
          format: presentationFormat,
        },
      ],
    },
  });
  const renderPassDescriptor = {
    label: "our basic canvas renderPass",
    colorAttachments: [
      {
        // view : <- 렌더링을 할 때 채워질 예정
        view: undefined as GPURenderPassColorAttachment["view"] | undefined,
        clearValue: [0.3, 0.3, 0.3, 1],
        loadOp: "clear",
        storeOp: "store",
      },
    ],
  };

  const render = () => {
    renderPassDescriptor.colorAttachments[0].view = context
      .getCurrentTexture()
      .createView();

    const encoder = device.createCommandEncoder({ label: "our encoder" });
    const pass = encoder.beginRenderPass(
      renderPassDescriptor as GPURenderPassDescriptor
    );
    pass.setPipeline(pipeline);
    pass.draw(3);
    pass.end();

    const commandBuffer = encoder.finish();
    device.queue.submit([commandBuffer]);
  };
  render();
}

main();
export {};
