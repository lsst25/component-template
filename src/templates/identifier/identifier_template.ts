import { IdentifierVoFactory } from "features/nmp-core";

const brand = Symbol("$pascal_name$EntityId");

export const $pascal_name$EntityId = IdentifierVoFactory.createNumber(brand);
export type $pascal_name$EntityId = InstanceType<typeof $pascal_name$EntityId>;
