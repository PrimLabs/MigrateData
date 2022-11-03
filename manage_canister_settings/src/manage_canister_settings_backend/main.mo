import Principal "mo:base/Principal";

shared(installer) actor class manage_canister_settings()  = this {

    public type CycleInterface = actor{
        withdraw_cycles : () -> async ();
    };

    public type canister_id = Principal;

    public type canister_settings = {
        freezing_threshold : ?Nat;
        controllers : ?[Principal];
        memory_allocation : ?Nat;
        compute_allocation : ?Nat;
    };

    public type definite_canister_settings = {
        freezing_threshold : Nat;
        controllers : [Principal];
        memory_allocation : Nat;
        compute_allocation : Nat;
    };

    public type user_id = Principal;

    public type wasm_module = [Nat8];

    public type Management = actor {

        canister_status : shared { canister_id : canister_id } -> async {
            status : { #stopped; #stopping; #running };
            memory_size : Nat;
            cycles : Nat;
            settings : definite_canister_settings;
            module_hash : ?[Nat8];
        };

        create_canister : shared { settings : ?canister_settings } -> async {
            canister_id : canister_id;
        };

        delete_canister : shared { canister_id : canister_id } -> async ();

        deposit_cycles : shared { canister_id : canister_id } -> async ();

        install_code : shared {
            arg : [Nat8];
            wasm_module : wasm_module;
            mode : { #reinstall; #upgrade; #install };
            canister_id : canister_id;
        } -> async ();

        provisional_create_canister_with_cycles : shared {
            settings : ?canister_settings;
            amount : ?Nat;
        } -> async { canister_id : canister_id };

        provisional_top_up_canister : shared {
            canister_id : canister_id;
            amount : Nat;
        } -> async ();

        raw_rand : shared () -> async [Nat8];

        start_canister : shared { canister_id : canister_id } -> async ();

        stop_canister : shared { canister_id : canister_id } -> async ();

        uninstall_code : shared { canister_id : canister_id } -> async ();

        update_settings : shared {
            canister_id : Principal;
            settings : canister_settings;
        } -> async ();
    };

    public shared({caller}) func updateControllers(canisterId: Principal): async Bool {
        let managementCanister: Management = actor("aaaaa-aa");
        await managementCanister.update_settings({
            canister_id = canisterId;
            settings = {
                freezing_threshold = null;
                controllers = ?[Principal.fromText("hjpa3-qyaaa-aaaan-qagva-cai")];
                memory_allocation = null;
                compute_allocation = null;
            }
        });
        true
    };

};