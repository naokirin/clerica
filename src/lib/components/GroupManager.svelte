<script lang="ts">
  import { groupsApi, type Group } from '$lib/api/groups';
  import { onMount } from 'svelte';
  import { Edit, Trash2 } from 'lucide-svelte';

  let groups: Group[] = [];
  let activeGroupId: string = '';
  let showCreateForm = false;
  let newGroupName = '';
  let editingGroupId: string | null = null;
  let editingGroupName = '';
  let loading = false;
  let error: string | null = null;

  onMount(async () => {
    try {
      await loadGroups();
      await loadActiveGroup();
    } catch (e) {
      console.error('初期化エラー:', e);
      error = e instanceof Error ? e.message : String(e);
    }
  });

  async function loadGroups() {
    try {
      console.log('グループ読み込み開始...');
      groups = await groupsApi.getGroups();
      console.log('グループ読み込み完了:', groups);
    } catch (error) {
      console.error('グループの読み込みに失敗しました:', error);
      throw error;
    }
  }

  async function loadActiveGroup() {
    try {
      console.log('アクティブグループ読み込み開始...');
      activeGroupId = await groupsApi.getActiveGroupId();
      console.log('アクティブグループ読み込み完了:', activeGroupId);
    } catch (error) {
      console.error('アクティブグループの読み込みに失敗しました:', error);
      throw error;
    }
  }

  async function switchGroup(groupId: string) {
    if (loading || groupId === activeGroupId) return;
    
    loading = true;
    try {
      await groupsApi.switchGroup(groupId);
      activeGroupId = groupId;
      
      // ページをリロードしてデータを更新
      window.location.reload();
    } catch (error) {
      console.error('グループの切り替えに失敗しました:', error);
      alert('グループの切り替えに失敗しました');
    } finally {
      loading = false;
    }
  }

  async function createGroup() {
    if (!newGroupName.trim()) return;

    try {
      await groupsApi.createGroup({ name: newGroupName.trim() });
      await loadGroups();
      newGroupName = '';
      showCreateForm = false;
    } catch (error) {
      console.error('グループの作成に失敗しました:', error);
      alert('グループの作成に失敗しました');
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
        name: editingGroupName.trim()
      });
      await loadGroups();
      editingGroupId = null;
      editingGroupName = '';
    } catch (error) {
      console.error('グループ名の更新に失敗しました:', error);
      alert('グループ名の更新に失敗しました');
    }
  }

  function cancelEdit() {
    editingGroupId = null;
    editingGroupName = '';
  }

  async function deleteGroup(groupId: string) {
    if (!confirm('このグループを削除しますか？グループ内のすべてのデータが削除されます。')) {
      return;
    }

    try {
      await groupsApi.deleteGroup(groupId);
      await loadGroups();
      await loadActiveGroup();
    } catch (error) {
      console.error('グループの削除に失敗しました:', error);
      alert('グループの削除に失敗しました: ' + error);
    }
  }
</script>

<div class="group-manager">
  
  {#if error}
    <div class="error-message">
      エラー: {error}
      <button on:click={() => {error = null; loadGroups(); loadActiveGroup();}}>再試行</button>
    </div>
  {/if}
  
  <div class="group-header">
    <h3>グループ管理</h3>
    <button 
      class="btn-create"
      on:click={() => showCreateForm = !showCreateForm}
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
        on:keydown={(e) => e.key === 'Enter' && createGroup()}
      />
      <div class="form-actions">
        <button class="btn-save" on:click={createGroup}>作成</button>
        <button class="btn-cancel" on:click={() => {showCreateForm = false; newGroupName = '';}}>
          キャンセル
        </button>
      </div>
    </div>
  {/if}

  <div class="groups-list">
    {#each groups as group (group.id)}
      <div class="group-item" class:active={group.id === activeGroupId}>
        {#if editingGroupId === group.id}
          <div class="edit-form">
            <input
              type="text"
              bind:value={editingGroupName}
              on:keydown={(e) => e.key === 'Enter' && saveEdit()}
            />
            <div class="edit-actions">
              <button class="btn-save" on:click={saveEdit}>保存</button>
              <button class="btn-cancel" on:click={cancelEdit}>キャンセル</button>
            </div>
          </div>
        {:else}
          <div class="group-content" on:click={() => switchGroup(group.id)}>
            <div class="group-info">
              <div class="group-name">{group.name}</div>
              <div class="group-status">
                {#if group.id === activeGroupId}
                  <span class="active-status">現在アクティブなグループ</span>
                {:else}
                  <span class="inactive-status">クリックして切り替え</span>
                {/if}
              </div>
            </div>
            <div class="group-actions">
              <button 
                class="icon-btn edit-btn" 
                on:click={(e) => {e.stopPropagation(); startEdit(group);}}
                disabled={loading}
                title="グループ名を編集"
              >
                <Edit size={14} />
              </button>
              <button 
                class="icon-btn delete-btn" 
                on:click={(e) => {e.stopPropagation(); deleteGroup(group.id);}}
                disabled={loading || groups.length <= 1}
                title="グループを削除"
              >
                <Trash2 size={14} />
              </button>
            </div>
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
  .group-manager {
    background: var(--bg-secondary, #f5f5f5);
    border-radius: 8px;
    padding: 16px;
    margin-bottom: 16px;
  }

  .group-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;
  }

  .group-header h3 {
    margin: 0;
    color: var(--text-primary);
  }

  .btn-create, .btn-save, .btn-cancel, .btn-edit, .btn-delete {
    padding: 6px 12px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 12px;
    transition: background-color 0.2s;
  }

  .btn-create {
    background: #007bff;
    color: white;
    border: 1px solid #007bff;
  }

  .btn-create:hover:not(:disabled) {
    background: #0056b3;
    border-color: #0056b3;
  }

  .btn-create:disabled {
    background: #6c757d;
    border-color: #6c757d;
    color: white;
    opacity: 0.6;
  }

  .btn-save {
    background: #28a745;
    color: white;
    border: 1px solid #28a745;
  }

  .btn-save:hover:not(:disabled) {
    background: #218838;
    border-color: #218838;
  }

  .btn-cancel {
    background: #6c757d;
    color: white;
    border: 1px solid #6c757d;
  }

  .btn-cancel:hover:not(:disabled) {
    background: #5a6268;
    border-color: #5a6268;
  }


  .create-form, .edit-form {
    background: var(--bg-primary);
    padding: 12px;
    border-radius: 6px;
    margin-bottom: 12px;
  }

  .create-form input, .edit-form input {
    width: 100%;
    padding: 8px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    margin-bottom: 8px;
    background: var(--bg-primary);
    color: var(--text-primary);
  }

  .form-actions, .edit-actions {
    display: flex;
    gap: 8px;
  }

  .groups-list {
    max-height: 300px;
    overflow-y: auto;
  }

  .group-item {
    border: 1px solid var(--border-color, #ddd);
    border-radius: 6px;
    margin-bottom: 8px;
    transition: border-color 0.2s, box-shadow 0.2s;
    background: var(--bg-primary, white);
  }

  .group-item.active {
    border-color: #007bff;
    background: rgba(0, 123, 255, 0.08);
    box-shadow: 0 2px 4px rgba(0, 123, 255, 0.15);
  }

  .group-item.active .group-name {
    color: #007bff;
    font-weight: 600;
  }

  .group-item.active .group-content:hover {
    background: rgba(0, 123, 255, 0.12);
  }

  .group-content {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .group-content:hover {
    background: rgba(0, 123, 255, 0.05);
  }

  .group-info {
    flex: 1;
    min-width: 0;
  }

  .group-name {
    font-weight: 500;
    color: var(--text-primary, #333);
    margin-bottom: 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .group-status {
    font-size: 11px;
    color: var(--text-muted, #666);
  }

  .active-status {
    color: #007bff;
    font-weight: 500;
  }

  .inactive-status {
    color: var(--text-muted, #666);
  }

  .group-actions {
    display: flex;
    gap: 4px;
    margin-left: 8px;
  }

  .icon-btn {
    padding: 4px;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    background: transparent;
    color: var(--text-muted, #666);
    transition: all 0.2s;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-btn:hover:not(:disabled) {
    background: rgba(0, 0, 0, 0.1);
  }

  .edit-btn:hover:not(:disabled) {
    color: #17a2b8;
    background: rgba(23, 162, 184, 0.1);
  }

  .delete-btn:hover:not(:disabled) {
    color: #dc3545;
    background: rgba(220, 53, 69, 0.1);
  }

  .icon-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .loading {
    text-align: center;
    color: var(--text-muted);
    font-style: italic;
    margin-top: 16px;
  }

  .error-message {
    background: #ffebee;
    color: #c62828;
    padding: 12px;
    border-radius: 6px;
    margin-bottom: 16px;
    border: 1px solid #ef5350;
  }

  .error-message button {
    margin-left: 8px;
    padding: 4px 8px;
    background: #c62828;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }
</style>