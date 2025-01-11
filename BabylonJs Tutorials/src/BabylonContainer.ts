import { Engine, Scene } from "babylonjs";

export class BabylonContainer {
  readonly engine: Engine;
  readonly scene: Scene;

  constructor(public readonly canvas: HTMLCanvasElement) {
    this.engine = new Engine(canvas, true);
    this.scene = new Scene(this.engine);
  }
}
