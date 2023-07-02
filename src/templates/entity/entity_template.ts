import { Opaque } from "../../common";

export type {entity}EntityId = Opaque<number, {entity}Entity>;

export interface {entity}Entity {
    id: {entity}EntityId;
}