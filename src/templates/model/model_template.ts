import { {pascal-name}Entity } from '../entities/{name}.entity';

export class {pascal-name}Model implements {pascal-name}Entity {
    public static create(): {pascal-name}Model {
        return new {pascal-name}Model();
    }

    private constructor() {
        Object.freeze(this);
    }
}