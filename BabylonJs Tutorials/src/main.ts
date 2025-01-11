import "./style.css";

import { BabylonContainer } from "./BabylonContainer";
import { base1 } from "./codes/Base1";

const canvas = document.getElementById("appCanvas") as HTMLCanvasElement;

const container = new BabylonContainer(canvas);

base1(container);
