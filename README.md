# MigrateData

1. 先开一个新的ICSP
2. 修改ICSP_canister_id参数。用此代码迁移数据，检查数据迁移是否成功。
3. 成功后，旧的ICSP安装manage_canister_settings代码，修改canister_controller,通过[iCAN](icantool.app)回收Cycles

- 此代码已经测试

1. Create a new ICSP first
2. Modify the ICSP_canister_id parameter. Use this code to migrate data and check if the data migration is successful.
3. After success, the old ICSP installs the manage_canister_settings code, modifies the canister_controller, and recycles Cycles through [iCAN](icantool.app)

- This code has been tested