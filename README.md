# MigrateData

1. 先开一个新的ICSP
2. 修改ICSP_canister_id参数。用此代码迁移数据，检查数据迁移是否成功。
3. 成功后，旧的ICSP先记录目前Bucket principal数组，安装manage_canister_settings代码，上传CycleWasm，使用delBucket删除各个Bucket，回收cycle到旧的ICSP中，然后命令行直接删除旧的ICSP回收所有cycle
(或者3. 成功后，旧的ICSP安装manage_canister_settings代码，修改canister_controller,通过[iCAN](https://icantool.app/)回收Bucket Cycles)

- 此代码已经测试

1. Create a new ICSP first
2. Modify the ICSP_canister_id parameter. Use this code to migrate data and check if the data migration is successful.
3. After success, the old ICSP first records the current bucket principal array, installs the manage_canister_settings code, uploads CycleWasm, uses delBucket to delete each bucket, recycles the cycle to the old ICSP, and then deletes the old ICSP directly from the command line to recycle all cycles

- This code has been tested
