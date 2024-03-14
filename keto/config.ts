import { Namespace, Context, SubjectSet } from "@ory/keto-namespace-types"

class User implements Namespace{
    related: {
        // related_category : RelatedCategory[]
        // where RelatedCategory impl NameSpace...
    }
    permits = {
        // see: (ctx:Context):boolean =>
        //  this.related.related_category.includes(ctx.Subject)
        //
    }
}

