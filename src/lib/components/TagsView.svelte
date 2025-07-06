<script lang="ts">
  import type { Tag } from "../types";
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
