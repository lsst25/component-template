import { $pascal_name$Entity } from '../entities/$name$.entity';

export class $pascal_name$Model implements $pascal_name$Entity {
  public static create(): $pascal_name$Model {
    return new $pascal_name$Model();
  }

  private constructor() {
      Object.freeze(this);
  }
}
