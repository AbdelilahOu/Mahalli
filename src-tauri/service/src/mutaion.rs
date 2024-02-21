use ::entity::prelude::*;
use sea_orm::*;

use crate::models::*;
pub struct MutationsService;

impl MutationsService {
    pub async fn create_product(db: &DbConn, product: NewProduct) -> Result<String, DbErr> {
        let product = ProductActiveModel {
            name: ActiveValue::Set(product.name),
            price: ActiveValue::Set(product.price),
            image: ActiveValue::Set(product.image),
            description: ActiveValue::Set(product.description),
            min_quantity: ActiveValue::Set(product.min_quantity),
            ..Default::default()
        };
        match product.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_product(db: &DbConn, product: Product) -> Result<(), DbErr> {
        let product_model = Products::find_by_id(product.id).one(db).await?;
        let mut product_active: ProductActiveModel = product_model.unwrap().into();
        product_active.name = ActiveValue::Set(product.name);
        product_active.price = ActiveValue::Set(product.price);
        product_active.image = ActiveValue::Set(product.image);
        product_active.description = ActiveValue::Set(product.description);
        product_active.min_quantity = ActiveValue::Set(product.min_quantity);
        match product_active.update(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_product(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let product_model = Products::find_by_id(id).one(db).await?;
        match product_model {
            Some(product_model) => {
                let product = product_model.delete(db).await?;
                Ok(product.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_client(db: &DbConn, client: NewClient) -> Result<String, DbErr> {
        let client = ClientActiveModel {
            full_name: ActiveValue::Set(client.full_name),
            email: ActiveValue::Set(client.email),
            phone_number: ActiveValue::Set(client.phone_number),
            address: ActiveValue::Set(client.address),
            image: ActiveValue::Set(client.image),
            ..Default::default()
        };
        match client.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_client(db: &DbConn, client: Client) -> Result<(), DbErr> {
        let client_model = Clients::find_by_id(client.id).one(db).await?;
        let mut client_active: ClientActiveModel = client_model.unwrap().into();
        client_active.full_name = ActiveValue::Set(client.full_name);
        client_active.email = ActiveValue::Set(client.email);
        client_active.phone_number = ActiveValue::Set(client.phone_number);
        client_active.address = ActiveValue::Set(client.address);
        client_active.image = ActiveValue::Set(client.image);
        match client_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_client(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let client_model = Clients::find_by_id(id).one(db).await?;
        match client_model {
            Some(client_model) => {
                let client = client_model.delete(db).await?;
                Ok(client.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_supplier(db: &DbConn, supplier: NewSupplier) -> Result<String, DbErr> {
        let supplier = SupplierActiveModel {
            full_name: ActiveValue::Set(supplier.full_name),
            email: ActiveValue::Set(supplier.email),
            phone_number: ActiveValue::Set(supplier.phone_number),
            address: ActiveValue::Set(supplier.address),
            image: ActiveValue::Set(supplier.image),
            ..Default::default()
        };
        match supplier.insert(db).await {
            Ok(p) => Ok(p.id),
            Err(err) => Err(err),
        }
    }
    pub async fn update_supplier(db: &DbConn, supplier: Supplier) -> Result<(), DbErr> {
        let supplier_model = Suppliers::find_by_id(supplier.id).one(db).await?;
        let mut supplier_active: SupplierActiveModel = supplier_model.unwrap().into();
        supplier_active.full_name = ActiveValue::Set(supplier.full_name);
        supplier_active.email = ActiveValue::Set(supplier.email);
        supplier_active.phone_number = ActiveValue::Set(supplier.phone_number);
        supplier_active.address = ActiveValue::Set(supplier.address);
        supplier_active.image = ActiveValue::Set(supplier.image);
        match supplier_active.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_supplier(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let supplier_model = Suppliers::find_by_id(id).one(db).await?;
        match supplier_model {
            Some(supplier_model) => {
                let supplier = supplier_model.delete(db).await?;
                Ok(supplier.rows_affected)
            }
            None => Ok(0),
        }
    }
    //
    pub async fn create_inv_mvm(db: &DbConn, mvm: NewInventory) -> Result<String, DbErr> {
        let in_mvm = InventoryMouvementActiveModel {
            mvm_type: ActiveValue::Set(mvm.mvm_type),
            quantity: ActiveValue::Set(mvm.quantity),
            product_id: ActiveValue::Set(mvm.product_id),
            ..Default::default()
        };
        match in_mvm.insert(db).await {
            Ok(im) => Ok(im.id),
            Err(err) => Err(err),
        }
    }
    pub async fn delete_inv_mvm(db: &DbConn, id: String) -> Result<u64, DbErr> {
        let city_model = InventoryMouvements::find_by_id(id).one(db).await?;
        match city_model {
            Some(city_model) => {
                let city = city_model.delete(db).await?;
                Ok(city.rows_affected)
            }
            None => Ok(0),
        }
    }
    pub async fn update_inv_mvm(db: &DbConn, q: f64) -> Result<(), DbErr> {
        // update only the quantity for now
        let mvm = InventoryMouvementActiveModel {
            quantity: ActiveValue::Set(q),
            ..Default::default()
        };

        match mvm.save(db).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}
