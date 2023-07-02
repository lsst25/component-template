import { {model}Entity } from '../entities/{model-name}.entity';

export class {model}Model implements {model}Entity {
    public static create(): {model}Model {
        return new {model}Model();
    }

    private constructor() {
        Object.freeze(this);
    }
}