<script lang="ts">
  import { shelvesApi, type Shelf } from "$lib/api/shelves";
  import { onMount } from "svelte";
  import { Edit, Trash2, Plus, PlusIcon } from "@lucide/svelte";
  import type { AppViewModel } from "$lib/viewmodels/AppViewModel";
  import Button from "../../parts/Button.svelte";
  import IconButton from "../../parts/IconButton.svelte";
  import { t } from "$lib/i18n";
  import TextInput from "../../parts/TextInput.svelte";

  interface Props {
    appViewModel?: AppViewModel;
  }

  let { appViewModel }: Props = $props();

  let shelves = $state<Shelf[]>([]);
  let activeShelfId = $state("");
  let showCreateForm = $state(false);
  let newShelfName = $state("");
  let editingShelfId = $state<string | null>(null);
  let editingShelfName = $state("");
  let loading = $state(false);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      await loadShelves();
      await loadActiveShelf();
    } catch (e) {
      console.error("初期化エラー:", e);
      error = e instanceof Error ? e.message : String(e);
    }
  });

  async function loadShelves() {
    try {
      shelves = await shelvesApi.getShelves();
    } catch (error) {
      console.error("シェルフの読み込みに失敗しました:", error);
      throw error;
    }
  }

  async function loadActiveShelf() {
    try {
      activeShelfId = await shelvesApi.getActiveShelfId();

      // AppViewModelにアクティブシェルフIDを設定
      if (appViewModel && activeShelfId) {
        appViewModel.switchShelf(activeShelfId);
      }
    } catch (error) {
      console.error("アクティブシェルフの読み込みに失敗しました:", error);
      throw error;
    }
  }

  async function switchShelf(shelfId: string) {
    if (loading || shelfId === activeShelfId) return;

    loading = true;
    try {
      await shelvesApi.switchShelf(shelfId);
      activeShelfId = shelfId;

      // AppViewModelを使ってデータを再読み込み
      if (appViewModel) {
        await appViewModel.switchShelf(shelfId);
      } else {
        // AppViewModelが利用できない場合はページリロード
        window.location.reload();
      }
    } catch (error) {
      console.error("シェルフの切り替えに失敗しました:", error);
      alert("シェルフの切り替えに失敗しました");
    } finally {
      loading = false;
    }
  }

  async function createShelf() {
    if (!newShelfName.trim()) return;

    try {
      await shelvesApi.createShelf({ name: newShelfName.trim() });
      await loadShelves();
      newShelfName = "";
      showCreateForm = false;
    } catch (error) {
      console.error("シェルフの作成に失敗しました:", error);
      alert("シェルフの作成に失敗しました");
    }
  }

  async function startEdit(shelf: Shelf) {
    editingShelfId = shelf.id;
    editingShelfName = shelf.name;
  }

  async function saveEdit() {
    if (!editingShelfId || !editingShelfName.trim()) return;

    try {
      await shelvesApi.updateShelfName({
        id: editingShelfId,
        name: editingShelfName.trim(),
      });
      await loadShelves();
      editingShelfId = null;
      editingShelfName = "";
    } catch (error) {
      console.error("シェルフ名の更新に失敗しました:", error);
      alert("シェルフ名の更新に失敗しました");
    }
  }

  function cancelEdit() {
    editingShelfId = null;
    editingShelfName = "";
  }

  async function deleteShelf(shelfId: string) {
    if (
      !confirm(
        "このシェルフを削除しますか？シェルフ内のすべてのデータが削除されます。",
      )
    ) {
      return;
    }

    try {
      await shelvesApi.deleteShelf(shelfId);
      await loadShelves();
      await loadActiveShelf();
    } catch (error) {
      console.error("シェルフの削除に失敗しました:", error);
      alert("シェルフの削除に失敗しました: " + error);
    }
  }
</script>

<div class="sidebar-section">
  {#if error}
    <div class="error-message">
      エラー: {error}
      <button
        onclick={() => {
          error = null;
          loadShelves();
          loadActiveShelf();
        }}>再試行</button
      >
    </div>
  {/if}

  <div class="section-header">
    <h3>シェルフ</h3>
    <Button
      onclick={() => (showCreateForm = !showCreateForm)}
      disabled={loading}
      iconName="Plus"
      text={$t("common.sidebar.createShelf")}
    />
  </div>

  {#if showCreateForm}
    <div class="create-form">
      <TextInput
        id="new-shelf-name-input"
        placeholder="シェルフ名を入力"
        bind:value={newShelfName}
        {...({ onkeydown: (e: KeyboardEvent) => e.key === "Enter" && createShelf() } as any)}
      />
      <div class="form-actions">
        <button class="btn-save" onclick={createShelf}>作成</button>
        <button
          class="btn-cancel"
          onclick={() => {
            showCreateForm = false;
            newShelfName = "";
          }}
        >
          キャンセル
        </button>
      </div>
    </div>
  {/if}

  <div class="shelf-list">
    {#each shelves as shelf (shelf.id)}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div
        class="shelf-item {shelf.id === activeShelfId ? 'selected' : ''}"
        onclick={() => switchShelf(shelf.id)}
        role="button"
        tabindex="0"
      >
        {#if editingShelfId === shelf.id}
          <div class="edit-form">
            <TextInput
              id="edit-shelf-name-input"
              bind:value={editingShelfName}
              {...({ onkeydown: (e: KeyboardEvent) => e.key === "Enter" && saveEdit() } as any)}
            />
            <div class="edit-actions">
              <button class="btn-save" onclick={saveEdit}>保存</button>
              <button class="btn-cancel" onclick={cancelEdit}
                >キャンセル</button
              >
            </div>
          </div>
        {:else}
          <div class="shelf-content">
            <div class="shelf-name">{shelf.name}</div>
            <div
              class="shelf-status {shelf.id === activeShelfId ? 'active' : ''}"
            >
              {#if shelf.id === activeShelfId}
                アクティブ
              {:else}
                クリックして切り替え
              {/if}
            </div>
          </div>
          <div class="shelf-actions">
            <IconButton
              icon={Edit}
              title="シェルフ名を編集"
              onClick={() => startEdit(shelf)}
              disabled={loading}
              class="green"
            />
            <IconButton
              icon={Trash2}
              title="シェルフを削除"
              onClick={() => deleteShelf(shelf.id)}
              disabled={loading || shelves.length <= 1}
              class="red"
            />
          </div>
        {/if}
      </div>
    {/each}
  </div>

  {#if loading}
    <div class="loading">処理中...</div>
  {/if}
</div>

<style>
  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.75rem;
  }

  .section-header h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
    color: #374151;
  }

  .btn-save,
  .btn-cancel {
    padding: 0.375rem 0.75rem;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    font-size: 0.75rem;
    transition: background-color 0.2s;
  }

  .btn-save {
    background: #10b981;
    color: white;
  }

  .btn-save:hover:not(:disabled) {
    background: #059669;
  }

  .btn-cancel {
    background: #6b7280;
    color: white;
  }

  .btn-cancel:hover:not(:disabled) {
    background: #4b5563;
  }

  .create-form,
  .edit-form {
    background: #f9fafb;
    padding: 0.75rem;
    border-radius: 0.25rem;
    margin-bottom: 0.75rem;
  }


  .form-actions,
  .edit-actions {
    display: flex;
    gap: 0.5rem;
  }

  .shelf-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .shelf-item {
    position: relative;
    margin-bottom: 0.25rem;
    background-color: #f9fafb;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    transition: background-color 0.2s;
    cursor: pointer;
    border: 1px solid transparent;
  }

  .shelf-item:hover {
    background-color: #f3f4f6;
  }

  .shelf-item.selected {
    background-color: #e0f2fe;
    border-left: 3px solid #3b82f6;
  }

  .shelf-item.selected .shelf-content {
    padding-left: 0.75rem;
  }

  .shelf-content {
    padding: 0.5rem;
    padding-right: 4rem;
    flex: 1;
    min-width: 0;
  }

  .shelf-name {
    font-weight: 500;
    color: #111827;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .shelf-status {
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.25rem;
    opacity: 0.8;
  }

  .shelf-status.active {
    color: #10b981;
    font-weight: 500;
  }

  .shelf-actions {
    position: absolute;
    top: 50%;
    right: 0.5rem;
    transform: translateY(-50%);
    display: flex;
    gap: 0.25rem;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .shelf-item:hover .shelf-actions {
    opacity: 1;
  }

  .loading {
    text-align: center;
    color: #6b7280;
    font-style: italic;
    margin-top: 1rem;
  }

  .error-message {
    background: #fee2e2;
    color: #dc2626;
    padding: 0.75rem;
    border-radius: 0.375rem;
    margin-bottom: 1rem;
    border: 1px solid #fecaca;
    font-size: 0.875rem;
  }

  .error-message button {
    margin-left: 0.5rem;
    padding: 0.25rem 0.5rem;
    background: #dc2626;
    color: white;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    font-size: 0.75rem;
  }
</style>
