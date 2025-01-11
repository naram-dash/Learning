import {
  ArcRotateCamera,
  HemisphericLight,
  MeshBuilder,
  Vector3,
} from "babylonjs";
import { BabylonContainer } from "../BabylonContainer";

export function base1(container: BabylonContainer) {
  const { canvas, engine, scene } = container;

  const camera = new ArcRotateCamera(
    "Camera",
    Math.PI / 2,
    Math.PI / 2,
    2,
    Vector3.Zero(),
    scene
  );
  camera.attachControl(true);

  const light1 = new HemisphericLight("light1", new Vector3(1, 1, 0), scene);
  // scene.addLight(light1);

  const sphere = MeshBuilder.CreateSphere("sphere", { diameter: 1 }, scene);

  // hide/show the Inspector
  window.addEventListener("keydown", (ev) => {
    // Shift+Ctrl+Alt+I
    if (ev.shiftKey && ev.ctrlKey && ev.altKey && ev.keyCode === 73) {
      if (scene.debugLayer.isVisible()) {
        scene.debugLayer.hide();
      } else {
        scene.debugLayer.show();
      }
    }
  });

  engine.runRenderLoop(() => {
    scene.render();
  });
}
