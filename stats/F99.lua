local RunService = game:GetService("RunService")
local CollectionService = game:GetService("CollectionService")
local ReplicatedStorage = game:GetService("ReplicatedStorage")
local BulletAssets = ReplicatedStorage:WaitForChild("Shared"):WaitForChild("Assets"):WaitForChild("Bullets")

local PartCache = require(ReplicatedStorage:WaitForChild("Shared"):WaitForChild("util"):WaitForChild("PartCache"))

-- all of the below tables, except the caches, are just enums
local GunTypes = {
    Auto = "Auto",
    Semi = "Semi",
}

local FireMode = {
    Single = "Single",
    Shotgun = "Shotgun",
    Burst = "Burst",
}

local BulletType = {
    Ray = "Ray",
    Lighting = "Lighting",
    Projectile = "Projectile",
}

local AmmoType = {
    Battery = "Battery",
    Ammo = "Ammo"
}

local Bullets = {
    Default = BulletAssets:WaitForChild("Default")
}

local Caches = {
    DefaultCache = nil
}

-- don't create extra parts that are just never used on the server
-- WeaponStats.Cache should never be touched on the server anyway
if RunService:IsClient() then
    CollectionService:AddTag(Bullets.Default, "Ignore")
    Caches.DefaultCache = PartCache.new(Bullets.Default, 200)
end

local Holsters = {
    Back = "Back",
    TorsoModule = "TorsoModule",
    Hip = "Hip",
    RightArmModule = "RightArmModule",
    LeftArmModule = "LeftArmModule",
    Melee = "Melee"
}

return {
	Name = "F99",
	FullName = "Plasma Thrower",
	Category = "Melee",
	Description = "The F99 flamethrower is a very bulky and unwieldy weapon, but one which carries great destructive power. Capable of firing forth a stream of critical temperature plasma at close range, any targets which are caught in the fire will find themselves quickly and assuredly roasted, completely vaporizing the target at worst.",
	QuickDescription = "Close quarters, constant area damage",
	WeaponCost = 1750,
	Slot = 1,
	Holster = Holsters.Back,
	NumHandles = 1,
	NumBarrels = 3,
	CanSprint = false,
	CanCrouch = false,
	WalkspeedReduce = 0,
	EquipTime = 0.5,
	BatteryDepletionMin = 4,
	BatteryDepletionMax = 5,
	ShotsDeplete = 4,
	MaxSpread = 1,
	MinSpread = 0.5,
	HeatRate = 2,
	CoolTime = 6,
	CoolWait = 1,
	CalculateDamage = function(damage, distance)
		damage = damage --+ (10 * (1/(distance*2)))
		return math.clamp(damage, 7, 50)
	end,
	VehicleMultiplier = 4,
	FireRate = 10,
	ChargeWait = 0,
	GunType = GunTypes.Auto,
	FireMode = FireMode.Flame,
	BulletType = BulletType.Flame,
	BulletModel = Bullets.Default,
	BulletCache = Caches.DefaultCache
,
	HandleWelds = {
		{	limb = "Right Arm",
			C0 = CFrame.new(.25, -2, -1.25) * CFrame.Angles(math.rad(-135),math.rad(180),0),
			C1 = CFrame.new()
		},
	},
}
