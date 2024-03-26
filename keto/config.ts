import { Namespace, Context } from "@ory/keto-namespace-types"

class Users {}


class Posts implements Namespace {
    related: {
        owners : Users[],
        editors: Users[],
    }
    permits = {
        edit: (ctx:Context):boolean => 
            this.related.owners.includes(ctx.Subject) ||
                this.related.editors.includes(ctx.Subject),
        view: (ctx:Context):boolean => this.permits.edit(ctx),
        add_editor: (ctx:Context):boolean =>
            this.related.owners.includes(ctx.Subject)
    }
}

