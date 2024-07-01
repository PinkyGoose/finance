//
// use crate::schema::expense::CreatedEntity;
// use crate::specific::expense;
// use crate::specific::expense::__path_create_expense;
// use crate::specific::expense::ExpenseQuery;
// use entities::{
//     expense::{CreateExpense, Model as Expense, UpdateExpense},
// };
// use utoipa::{openapi::Server, Modify, OpenApi};
//
// #[derive(utoipa::ToSchema)]
// struct Value {}
//
// #[derive(utoipa::ToSchema)]
// struct Json {}
// #[derive(OpenApi)]
// #[openapi(
//     paths(
//
//         expense::create_expense,
//     ),
//     components(
//         schemas(
//             CreatedEntity,
//             AffectedRows,
//             Value,
//             Json,
//             ModuleListQuery,
//             Count,
//             Empty,
//
//         // zones
//             SingleZone,
//             ZoneWithNested,
//             ModuleSummary,
//             Zone,
//             UpdateZone,
//             CreateZone,
//
//         // groups
//             Group,
//             CreateGroup,
//
//         // health
//             CreateModuleState,
//             CreateNodeState,
//             CreateNodeMetrics,
//             ModuleState,
//             NodeState,
//             NodeMetrics,
//             ModuleStateResponse,
//             NodeStateResponse,
//
//         // layouts
//             FavouriteLayout,
//             Layout,
//             CreateLayout,
//             UpdateLayout,
//
//         // modules
//             Module,
//             ModuleList,
//             UpdateModule,
//             CreateModule,
//
//         // nodes
//             Node,
//             NodeList,
//             NodeConfig,
//             NodeMini,
//             ModuleInNode,
//             CreateNode,
//             UpdateNode,
//
//         // service
//             Config,
//
//         // obectlinks
//             CreateObjectLink, ObjectLink, LinksFilter, LinksByMemberFilter, EditObjectLink,
//
//         // history
//             RecordsCount, Filter, TimeRange, RecordLimit, RecordSort, CreateHistoryRec, HistoryList,
//
//         // incident
//             Incident, IncidentList, CreateIncident, IncidentDone, IncidentFilter, IncidentInProgress, IncidentWithModule, IncidentDateType, IncidentCount, FavouriteIncident,
//
//         // role
//             Role,
//
//         // arm
//             Arm, ArmQuery, CreateArm, UpdateArm,
//         //arm_type
//             ArmType,
//         // Users
//             User, UserQuery, CreateUser, UpdateUser, UserRole, SignInUser, AuthUser, Ordering, UserOrdering,
//         // Patrol
//             Patrol, CreatePatrol, UpdatePatrol, PatrolListQuery,
//         // PresetImage
//             PresetImage, CreatePresetImage, UpdatePresetImage, PresetImageListQuery, PresetImageSmall,
//         // Panorama
//             CreatePanorama, PanoramaImage, PanoramaMeta,
//
//         // подписки
//             SubscriptionAction,
//             NodeListSub,
//             NodeListSub,
//             NodeSub,
//             ModuleSub,
//             ModuleListSub,
//             ArmListSub,
//             HistoryListSub,
//             IncidentListSub,
//             UserSub,
//         )
//     ),
//     modifiers(&ServerAddon),
// )]
//
// pub struct Docs;
// struct ServerAddon;
//
// impl Modify for ServerAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         openapi.servers = Some(vec![Server::new("/api/data")])
//     }
// }
