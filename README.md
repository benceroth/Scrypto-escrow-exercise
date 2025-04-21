# Test solution through resim for tokens
resim reset
$OP1=$(resim new-account)
$ACC_ADDRESS=$OP1 -match "Account component address: ()" -split ' ' | select-object -last 1
$ACC_BADGE=$OP1 -match "Owner badge: ()" -split ' ' | select-object -last 1

$OP1=resim show
$ACC_RADIX=$OP1 -match "() Radix" | %{ $_ -replace ":", "" -split ' ' | select -index 1}

$PK_OP=$(resim publish ".")
$PACKAGE=$PK_OP -split ' ' | select-object -last 1

$TOKEN_COMPONENT="Escrow"
$COMPONENT_OP = resim call-function $PACKAGE $TOKEN_COMPONENT instantiate_fungible $ACC_RADIX 1 "$($ACC_RADIX):5" --proofs $ACC_BADGE
$COMPONENT_ADDRESS=$COMPONENT_OP -match "Component: ()" -split ' ' | select-object -last 1

$OP1=resim show
$ACC_ESCROW=$OP1 -match "() \?" | %{ $_ -replace ":", "" -split ' ' | select -index 1}

resim call-method $COMPONENT_ADDRESS exchange "$($ACC_RADIX):1"
resim call-method $COMPONENT_ADDRESS withdraw_resource "$($ACC_ESCROW):1"
resim call-method $COMPONENT_ADDRESS cancel_escrow "$($ACC_ESCROW):1"

## Create a bucket of 100 Radix
CALL_METHOD 
    Address("account") 
    "withdraw"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("100");

TAKE_FROM_WORKTOP
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("100")
    Bucket("bucket1");

## Create the component by requesting 150 Radix for our 100 Radix
CALL_FUNCTION
    Address("package")
    "Escrow"
    "instantiate_fungible"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("150")
    Bucket("bucket1");

## Store the returned badge into our account
CALL_METHOD
    Address("account")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");

## Create a bucket of 150 Radix
CALL_METHOD 
    Address("account") 
    "withdraw"
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("150");

TAKE_FROM_WORKTOP
    Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
    Decimal("150")
    Bucket("bucket1");

## Make the best exchange of our life by sending 150 Radix for 100 Radix
CALL_METHOD
    Address("component")
    "exchange"
    Bucket("bucket1");

## Store 100 Radix into our account
CALL_METHOD
    Address("account")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");

## Withdraw the 150 Radix:
CALL_METHOD 
    Address("account") 
    "withdraw"
    Address("resource_tdx_2_1n2r0pzenxw7zv6jzds7aaxdxh3zxxxm48shkz5tea2nkdztedkj5eh")
    Decimal("1");

TAKE_FROM_WORKTOP
    Address("resource_tdx_2_1n2r0pzenxw7zv6jzds7aaxdxh3zxxxm48shkz5tea2nkdztedkj5eh")
    Decimal("1")
    Bucket("bucket1");

## Make the best withdraw of our life by getting 150 Radix for 100 Radix
CALL_METHOD
    Address("component")
    "withdraw_resource"
    Bucket("bucket1");

## Store 150 Radix into our account
CALL_METHOD
    Address("account")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP");
