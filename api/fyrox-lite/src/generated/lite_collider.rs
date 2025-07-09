
            pub struct LiteConeShape
            {
                pub half_height: f32,
            
                pub radius: f32,
            
            }
            
            pub struct LiteHeightfieldShape
            {
                pub geometry_source: LiteGeometrySource,
            
            }
            
            pub struct LiteTrimeshShape
            {
                pub sources: Vec<LiteGeometrySource>,
            
            }
            
            pub struct LiteInteractionGroups
            {
                pub memberships: LiteBitMask,
            
                pub filter: LiteBitMask,
            
            }
            
            pub struct LiteConvexPolyhedronShape
            {
                pub geometry_source: LiteGeometrySource,
            
            }
            
            pub struct LiteCylinderShape
            {
                pub half_height: f32,
            
                pub radius: f32,
            
            }
            
            pub struct LiteBallShape
            {
                pub radius: f32,
            
            }
            