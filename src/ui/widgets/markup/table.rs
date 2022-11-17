use crate::rich_text::Table;
use druid::widget::RawLabel;
use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LensExt, LifeCycle, LifeCycleCtx,
    PaintCtx, Size, UpdateCtx, Widget, WidgetExt, WidgetPod,
};
use druid_widget_nursery::table::{FlexTable, TableRow};

pub struct MarkupTable {
    table: WidgetPod<Table, FlexTable<Table>>,
}

impl MarkupTable {
    pub fn new() -> Self {
        Self {
            table: WidgetPod::new(FlexTable::new()),
        }
    }

    fn update_table(&mut self, data: &Table) {
        let table = self.table.widget_mut();
        let mut column_count = None;
        for row in &data.rows {
            if let Some(count) = column_count {
                if count != row.fields.len() {
                    return;
                }
            } else {
                column_count = Some(row.fields.len());
            }
        }
        for (row_index, row) in data.rows.iter().enumerate() {
            let mut table_row = TableRow::new();
            for (field, _) in row.fields.iter().enumerate() {
                table_row.add_child(
                    RawLabel::new().lens(
                        druid::lens!(Table, rows[row_index])
                            .then(druid::lens!(crate::rich_text::TableRow, fields[field]))
                            .then(crate::rich_text::TableField::content),
                    ),
                );
            }
            table.add_row(table_row);
        }
    }
}

impl Widget<Table> for MarkupTable {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Table, env: &Env) {
        self.table.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Table, env: &Env) {
        if matches!(event, LifeCycle::WidgetAdded) {
            self.update_table(data);
        }
        self.table.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Table, data: &Table, env: &Env) {
        if !old_data.same(data) {
            self.update_table(data);
        }
        self.table.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Table,
        env: &Env,
    ) -> Size {
        self.table.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Table, env: &Env) {
        self.table.paint(ctx, data, env);
    }
}
