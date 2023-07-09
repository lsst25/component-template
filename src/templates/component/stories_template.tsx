import { {pascal-name} } from "./{name}.component";

export default {
  title: "_____/{pascal-name}",
  component: {pascal-name},
};

export const Base = (props) => <{pascal-name} {...props} />;
Base.args = {};