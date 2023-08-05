import type { Meta, StoryObj } from "@storybook/react";

import { $pascal_name$ } from "./$name$.component";

const meta: Meta<typeof $pascal_name$> = {
  title: "_____/$pascal_name$",
  component: $pascal_name$,
};

export default meta;
type Story = StoryObj<typeof $pascal_name$>;

export const Base: Story = {
  args: {},
  render: (args) => <$pascal_name$ {...args} />,
};

