## Creating a Storybook Story

Every component in `src/lib/components/` must have a matching `*.stories.svelte` file.

### 1. Create the file

`src/lib/components/MyComponent.stories.svelte`

### 2. Structure

```svelte
<script lang="ts" module>
  import { defineMeta } from '@storybook/addon-svelte-csf';
  import MyComponent from './MyComponent.svelte';

  const { Story } = defineMeta({
    component: MyComponent,
    tags: ['autodocs'],
    argTypes: {
      isShow: { control: 'boolean' },
      title: { control: 'text' },
    },
  });
</script>
```

### 3. Add stories (one per visual state)

Simple props:

```svelte
<Story name="Default" args={{ label: 'Hello', count: 5 }}>
  {#snippet template(args)}
    <MyComponent {...args} />
  {/snippet}
</Story>

<Story name="Disabled" args={{ label: 'Hello', count: 0, disabled: true }}>
  {#snippet template(args)}
    <MyComponent {...args} />
  {/snippet}
</Story>
```

Components with snippets (children, content):

```svelte
<Story name="Open" args={{ isShow: true, onclose: () => {}, title: 'Title' }}>
  {#snippet template(args)}
    <MyComponent isShow={args.isShow} onclose={args.onclose} title={args.title}>
      {#snippet children()}
        <p>Body content here.</p>
      {/snippet}
    </MyComponent>
  {/snippet}
</Story>
```

### 4. Run

```bash
yarn storybook
```

Hot-reloads on save. Use the Controls panel to interactively tweak args.
