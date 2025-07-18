<script lang="ts">
  import { t } from "$lib/i18n";

  interface Variable {
    name: string;
    description: string;
  }

  interface Props {
    variables?: Variable[];
    showRegexHelp?: boolean;
  }

  let {
    variables = [
      {
        name: `{{ file.name }}`,
        description: $t("common.fileDetail.fullFilename"),
      },
      {
        name: `{{ file.name_stem() }}`,
        description: $t("common.fileDetail.filenameWithoutExtension"),
      },
      {
        name: `{{ file.extension() }}`,
        description: $t("common.fileDetail.fileExtension"),
      },
      {
        name: `{{ tags | join(sep="_") }}`,
        description: $t("common.fileDetail.tagsJoined"),
      },
      {
        name: `{{ metadata | join(sep="_") }}`,
        description: $t("common.fileDetail.metadata"),
      },
      {
        name: `{{ file.created_at | date(format="%Y-%m-%d") }}`,
        description: "作成日時",
      },
      { name: `{{ n }}`, description: "連番（1から開始）" },
    ],
    showRegexHelp = true,
  }: Props = $props();
</script>

<div class="rename-help">
  <details>
    <summary>{$t("common.fileDetail.renameHelp")}</summary>
    <div class="help-content">
      {#if variables.length > 0}
        <h5>{$t("common.fileDetail.availableVariables")}:</h5>
        <ul>
          {#each variables as variable}
            <li><code>{variable.name}</code> - {variable.description}</li>
          {/each}
        </ul>
      {/if}

      {#if showRegexHelp}
        <h5>{$t("common.fileDetail.regexBackreferences")}:</h5>
        <p>$1, $2, $3... - {$t("common.fileDetail.regexBackreferencesDesc")}</p>
      {/if}
    </div>
  </details>
</div>

<style>
  .rename-help {
    margin-top: 0.5rem;
  }

  .rename-help details summary {
    cursor: pointer;
    font-size: 0.875rem;
    color: #007acc;
    margin-bottom: 0.5rem;
  }

  .help-content {
    background-color: white;
    padding: 1rem;
    border: 1px solid #dee2e6;
    border-radius: 4px;
    font-size: 0.875rem;
  }

  .help-content h5 {
    margin: 0 0 0.5rem 0;
    color: #495057;
    font-size: 0.875rem;
  }

  .help-content ul {
    margin: 0 0 1rem 0;
    padding-left: 1.5rem;
  }

  .help-content li {
    margin-bottom: 0.25rem;
  }

  .help-content code {
    background-color: #f8f9fa;
    padding: 0.125rem 0.25rem;
    border-radius: 3px;
    font-family: "Monaco", "Menlo", "Ubuntu Mono", monospace;
    font-size: 0.8rem;
    color: #e83e8c;
  }

  .help-content p {
    margin: 0;
    color: #6c757d;
  }
</style>
