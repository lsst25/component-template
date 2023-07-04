import { Opaque } from "features/common";

export type {pascal-name}EntityId = Opaque<number, {pascal-name}Entity>;

export interface {pascal-name}Entity {
    id: {pascal-name}EntityId;
}