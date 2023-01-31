use re_log_types::ComponentPath;

use super::{component::arrow_component_ui, DataUi};

impl DataUi for ComponentPath {
    fn data_ui(
        &self,
        ctx: &mut crate::misc::ViewerContext<'_>,
        ui: &mut egui::Ui,
        verbosity: crate::ui::UiVerbosity,
        query: &re_arrow_store::LatestAtQuery,
    ) {
        ui.horizontal(|ui| {
            ui.label("Entity path:");
            ctx.entity_path_button(ui, None, &self.entity_path);
        });

        let store = &ctx.log_db.entity_db.arrow_store;

        match re_query::get_component_with_instances(
            store,
            query,
            self.entity_path(),
            self.component_name,
        ) {
            Err(re_query::QueryError::PrimaryNotFound) => {
                ui.label("<unset>");
            }
            // Any other failure to get a component is unexpected
            Err(err) => {
                ui.label(format!("Error: {}", err));
            }
            Ok(component_data) => {
                arrow_component_ui(ctx, ui, &component_data, verbosity, query);
            }
        }
    }
}