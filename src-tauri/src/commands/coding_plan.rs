use crate::store::AppState;
use tauri::State;

use crate::services::subscription::SubscriptionQuota;

#[tauri::command]
pub async fn get_coding_plan_quota(
    state: State<'_, AppState>,
    base_url: String,
    api_key: String,
    // 火山方舟用控制面 AK/SK 签名查询用量；其他供应商不传，沿用 api_key。
    access_key_id: Option<String>,
    secret_access_key: Option<String>,
    // 智谱团队版（zhipu_team）靠显式标识路由（base_url 与个人版相同无法区分）。
    coding_plan_provider: Option<String>,
    team_organization_id: Option<String>,
    team_project_id: Option<String>,
) -> Result<SubscriptionQuota, String> {
    // 火山方舟 AK/SK 自动复用：弹窗「测试查询」场景下当前条目可能尚未配置，
    // 与后台刷新路径一致地回填其他火山条目已配置的凭据。
    let base_lc = base_url.to_lowercase();
    let is_volcengine =
        base_lc.contains("volces.com/api/plan") || base_lc.contains("volces.com/api/coding");
    let (access_key_id, secret_access_key) = if is_volcengine {
        crate::services::coding_plan::resolve_volcengine_aksk_with_fallback(
            &state,
            access_key_id.as_deref(),
            secret_access_key.as_deref(),
        )
    } else {
        (access_key_id, secret_access_key)
    };
    crate::services::coding_plan::get_coding_plan_quota(
        &base_url,
        &api_key,
        access_key_id.as_deref(),
        secret_access_key.as_deref(),
        coding_plan_provider.as_deref(),
        team_organization_id.as_deref(),
        team_project_id.as_deref(),
    )
    .await
}
