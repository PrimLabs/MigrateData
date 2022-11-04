import Principal "mo:base/Principal";

shared(installer) actor class manage_canister_settings()  = this {

    private let management: Management = actor("aaaaa-aa");
    private stable var cycle_wasm : [Nat8] = [];

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

    public shared({caller}) func installCycleWasm(wasm : [Nat8]) : async Bool {
        cycle_wasm := wasm;
        true
    };

    public shared({caller}) func delBucket(bucket : Principal) : async Bool {
        let bal = (await management.canister_status({ canister_id = bucket })).cycles;
        await management.start_canister({ canister_id = bucket });
        await management.install_code({
            arg = [];
            wasm_module = cycle_wasm;
            mode = #reinstall;
            canister_id = bucket;
        });
        let from: CycleInterface = actor(Principal.toText(bucket));
        await from.withdraw_cycles();
        await management.stop_canister({ canister_id = bucket });
        ignore management.delete_canister({ canister_id = bucket });
        true
    };

    public shared({caller}) func updateControllers(canisterId: Principal, controller: Principal): async Bool {
        await management.update_settings({
            canister_id = canisterId;
            settings = {
                freezing_threshold = null;
                controllers = ?[controller];
                memory_allocation = null;
                compute_allocation = null;
            }
        });
        true
    };

};
