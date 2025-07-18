<script lang="ts">
  import type { Tag } from "../../../types";
  import { t } from "$lib/i18n";

  interface Props {
    tags: Tag[];
  }

  let { tags }: Props = $props();
</script>

<div class="tags-view">
  <h2>{$t("common.tags.title")}</h2>
  <div class="tags-grid">
    {#each tags as tag (tag.id)}
      <div class="tag-card" style="border-color: {tag.color}">
        <div class="tag-name">{tag.name}</div>
        <div class="tag-color" style="background-color: {tag.color}"></div>
      </div>
    {/each}
  </div>
</div>

<style>
  .tags-view {
    max-width: 800px;
    margin: 0 auto;
  }

  .tags-view h2 {
    margin: 0 0 1.5rem 0;
    font-size: 1.5rem;
    font-weight: 600;
    color: #111827;
  }

  .tags-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 1rem;
  }

  .tag-card {
    padding: 1rem;
    background-color: white;
    border: 2px solid;
    border-radius: 0.5rem;
    text-align: center;
  }

  .tag-name {
    font-weight: 500;
    color: #111827;
  }

  .tag-color {
    width: 100%;
    height: 20px;
    border-radius: 0.25rem;
  }
</style>
