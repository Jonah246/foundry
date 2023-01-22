pragma solidity >=0.8.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";


contract TargetContract {

    function execute() public returns(uint256) {
        return 42;
    }
}

contract SimpleHookTest is DSTest {
    Cheats constant cheats = Cheats(HEVM_ADDRESS);
    TargetContract target;
    function setUp() public {
        target = new TargetContract();
    }

    function hook(address target, bytes calldata input) public returns(uint256) {
        cheats.prank(msg.sender);
        (bool success, bytes memory value) = target.call(input);
        require(success, "hook call failed");
        return abi.decode(value, (uint256)) + 250;
    }

    /// Tests that simple hook that does not touch storage would works
    function testHook() public {
        cheats.hookCall(
            address(target),
            abi.encodeWithSignature("execute()"),
            abi.encode(this.hook.selector)
        );
        require(target.execute() == 42 + 250, "hook failed");
    }
}
