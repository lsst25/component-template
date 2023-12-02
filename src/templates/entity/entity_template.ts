import * as z from "zod";

import { Entity } from "features/nmp-core";

import type { $pascal_name$EntityId } from "../vos";

const propsSchema = z.object({
  name: z.string(),
});

export type $pascal_name$EntityProps = z.infer<typeof propsSchema>;

export class $pascal_name$Entity extends Entity<$pascal_name$EntityProps, $pascal_name$EntityId> {
  static create(props: $pascal_name$EntityProps, id: $pascal_name$EntityId): $pascal_name$Entity {
    this.validate(props);

    return new $pascal_name$Entity(props, id);
  }

  static propsSchema = propsSchema;

  static validate(props: $pascal_name$EntityProps): void {
    this.propsSchema.parse(props);
  }

  get name(): string {
    return this.getProperty("name");
  }

  get key(): string {
    return this.id.toString();
  }
}
