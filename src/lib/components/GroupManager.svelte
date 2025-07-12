<script lang="ts">
  import { groupsApi, type Group } from "$lib/api/groups";
  import { onMount } from "svelte";
  import { Edit, Trash2 } from "lucide-svelte";
  import type { AppViewModel } from "$lib/viewmodels/AppViewModel";

  interface Props {
    appViewModel?: AppViewModel;
  }

  let { appViewModel }: Props = $props();

  let groups = $state<Group[]>([]);
  let activeGroupId = $state("");
  let showCreateForm = $state(false);
  let newGroupName = $state("");
  let editingGroupId = $state<string | null>(null);
  let editingGroupName = $state("");
  let loading = $state(false);
  let error = $state<string | null>(null);

  onMount(async () => {
    try {
      await loadGroups();
      await loadActiveGroup();
    } catch (e) {
      console.error("初期化エラー:", e);
      error = e instanceof Error ? e.message : String(e);
    }
  });

  async function loadGroups() {
    try {
      console.log("グループ読み込み開始...");
      groups = await groupsApi.getGroups();
      console.log("グループ読み込み完了:", groups);
    } catch (error) {
      console.error("グループの読み込みに失敗しました:", error);
      throw error;
    }
  }

  async function loadActiveGroup() {
    try {
      console.log("アクティブグループ読み込み開始...");
      activeGroupId = await groupsApi.getActiveGroupId();
      console.log("アクティブグループ読み込み完了:", activeGroupId);
    } catch (error) {
      console.error("アクティブグループの読み込みに失敗しました:", error);
      throw error;
    }
  }

  async function switchGroup(groupId: string) {
    if (loading || groupId === activeGroupId) return;

    loading = true;
    try {
      await groupsApi.switchGroup(groupId);
      activeGroupId = groupId;

      // AppViewModelを使ってデータを再読み込み
      if (appViewModel) {
        await appViewModel.switchGroup(groupId);
      } else {
        // AppViewModelが利用できない場合はページリロード
        window.location.reload();
      }
    } catch (error) {
      console.error("グループの切り替えに失敗しました:", error);
      alert("グループの切り替えに失敗しました");
    } finally {
      loading = false;
    }
  }

  async function createGroup() {
    if (!newGroupName.trim()) return;

    try {
      await groupsApi.createGroup({ name: newGroupName.trim() });
      await loadGroups();
      newGroupName = "";
      showCreateForm = false;
    } catch (error) {
      console.error("グループの作成に失敗しました:", error);
      alert("グループの作成に失敗しました");
    }
  }

  async function startEdit(group: Group) {
    editingGroupId = group.id;
    editingGroupName = group.name;
  }

  async function saveEdit() {
    if (!editingGroupId || !editingGroupName.trim()) return;

    try {
      await groupsApi.updateGroupName({
        id: editingGroupId,
        name: editingGroupName.trim(),
      });
      await loadGroups();
      editingGroupId = null;
      editingGroupName = "";
    } catch (error) {
      console.error("グループ名の更新に失敗しました:", error);
      alert("グループ名の更新に失敗しました");
    }
  }

  function cancelEdit() {
    editingGroupId = null;
    editingGroupName = "";
  }

  async function deleteGroup(groupId: string) {
    if (
      !confirm(
        "このグループを削除しますか？グループ内のすべてのデータが削除されます。",
      )
    ) {
      return;
    }

    try {
      await groupsApi.deleteGroup(groupId);
      await loadGroups();
      await loadActiveGroup();
    } catch (error) {
      console.error("グループの削除に失敗しました:", error);
      alert("グループの削除に失敗しました: " + error);
    }
  }
</script>

<div class="sidebar-section">
  {#if error}
    <div class="error-message">
      エラー: {error}
      <button
        on:click={() => {
          error = null;
          loadGroups();
          loadActiveGroup();
        }}>再試行</button
      >
    </div>
  {/if}

  <div class="section-header">
    <h3>グループ管理</h3>
    <button
      class="add-button"
      on:click={() => (showCreateForm = !showCreateForm)}
      disabled={loading}
    >
      新規作成
    </button>
  </div>

  {#if showCreateForm}
    <div class="create-form">
      <input
        type="text"
        placeholder="グループ名を入力"
        bind:value={newGroupName}
        on:keydown={(e) => e.key === "Enter" && createGroup()}
      />
      <div class="form-actions">
        <button class="btn-save" on:click={createGroup}>作成</button>
        <button
          class="btn-cancel"
          on:click={() => {
            showCreateForm = false;
            newGroupName = "";
          }}
        >
          キャンセル
        </button>
      </div>
    </div>
  {/if}

  <div class="group-list">
    {#each groups as group (group.id)}
      <div
        class="group-item {group.id === activeGroupId ? 'selected' : ''}"
        on:click={() => switchGroup(group.id)}
      >
        {#if editingGroupId === group.id}
          <div class="edit-form">
            <input
              type="text"
              bind:value={editingGroupName}
              on:keydown={(e) => e.key === "Enter" && saveEdit()}
            />
            <div class="edit-actions">
              <button class="btn-save" on:click={saveEdit}>保存</button>
              <button class="btn-cancel" on:click={cancelEdit}
                >キャンセル</button
              >
            </div>
          </div>
        {:else}
          <div class="group-content">
            <div class="group-name">{group.name}</div>
            <div
              class="group-status {group.id === activeGroupId ? 'active' : ''}"
            >
              {#if group.id === activeGroupId}
                アクティブ
              {:else}
                クリックして切り替え
              {/if}
            </div>
          </div>
          <div class="group-actions">
            <button
              class="edit-btn"
              on:click={(e) => {
                e.stopPropagation();
                startEdit(group);
              }}
              disabled={loading}
              title="グループ名を編集"
            >
              <Edit size={14} />
            </button>
            <button
              class="delete-btn"
              on:click={(e) => {
                e.stopPropagation();
                deleteGroup(group.id);
              }}
              disabled={loading || groups.length <= 1}
              title="グループを削除"
            >
              <Trash2 size={14} />
            </button>
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

  .add-button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.75rem;
    background-color: #3b82f6;
    color: white;
    border: none;
    border-radius: 0.375rem;
    font-size: 0.875rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .add-button:hover:not(:disabled) {
    background-color: #2563eb;
  }

  .add-button:disabled {
    background-color: #9ca3af;
    cursor: not-allowed;
    opacity: 0.6;
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

  .create-form input,
  .edit-form input {
    width: 100%;
    padding: 0.5rem;
    border: 1px solid #d1d5db;
    border-radius: 0.25rem;
    margin-bottom: 0.5rem;
    background: white;
    color: #111827;
  }

  .form-actions,
  .edit-actions {
    display: flex;
    gap: 0.5rem;
  }

  .group-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .group-item {
    position: relative;
    margin-bottom: 0.25rem;
    background-color: #f9fafb;
    border-radius: 0.25rem;
    font-size: 0.875rem;
    transition: background-color 0.2s;
    cursor: pointer;
    border: 1px solid transparent;
  }

  .group-item:hover {
    background-color: #f3f4f6;
  }

  .group-item.selected {
    background-color: #e0f2fe;
    border-left: 3px solid #3b82f6;
  }

  .group-item.selected .group-content {
    padding-left: 0.75rem;
  }

  .group-content {
    padding: 0.5rem;
    padding-right: 4rem;
    flex: 1;
    min-width: 0;
  }

  .group-name {
    font-weight: 500;
    color: #111827;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .group-status {
    font-size: 0.75rem;
    color: #6b7280;
    margin-top: 0.25rem;
    opacity: 0.8;
  }

  .group-status.active {
    color: #10b981;
    font-weight: 500;
  }

  .group-actions {
    position: absolute;
    top: 50%;
    right: 0.5rem;
    transform: translateY(-50%);
    display: flex;
    gap: 0.25rem;
    opacity: 0;
    transition: opacity 0.2s;
  }

  .group-item:hover .group-actions {
    opacity: 1;
  }

  .edit-btn,
  .delete-btn {
    padding: 0.25rem;
    border: none;
    border-radius: 0.25rem;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.2s;
  }

  .edit-btn {
    background-color: #10b981;
    color: white;
  }

  .edit-btn:hover:not(:disabled) {
    background-color: #059669;
  }

  .delete-btn {
    background-color: #ef4444;
    color: white;
  }

  .delete-btn:hover:not(:disabled) {
    background-color: #dc2626;
  }

  .edit-btn:disabled,
  .delete-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
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
