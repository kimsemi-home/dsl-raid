import type { Point } from "./geometry";
import type { StyleToken } from "./style";

export type SceneNode = {
  id: string;
  subject: string;
  x: number;
  y: number;
  width: number;
  height: number;
  label: string;
  badges: string[];
  style?: StyleToken;
};

export type SceneEdge = {
  id: string;
  subject: string;
  from: string;
  to: string;
  label?: string;
  route: Point[];
  style?: StyleToken;
};
