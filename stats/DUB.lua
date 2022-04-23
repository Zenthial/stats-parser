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
	Name = "DUB",
	FullName = "",
	Category = "",
	Description = "The DUB Rifle is a specialized variant of the E31 experimental electron rifle. Like it's predecessor, the rifle utilizes a system of subatomic levers to channel a battery’s power into an ultra high amperage and voltage electron bolt. This particular variant curiously includes a specialized audio module that will play music.",
	QuickDescription = "DUB WUB WUB WUB",
	WeaponCost = 4500,
	Slot = 1,
	Holster = Holsters.Back,
	NumHandles = 1,
	NumBarrels = 2,
	CanSprint = true,
	CanCrouch = true,
	HeadshotMultiplier = 2,
	WalkspeedReduce = 0,
	EquipTime = 0.3,
	BatteryDepletionMin = 1,
	BatteryDepletionMax = 1,
	ShotsDeplete = 20,
	MaxSpread = 1.5,
	MinSpread = 0.5,
	HeatRate = 0.1,
	CoolTime = 2,
	CoolWait = 0.5,
	CalculateDamage = function(damage, distance)
		return damage
	end,
	VehicleMultiplier = 4,
	FireRate = 20,
	ChargeWait = 0.3,
	GunType = GunTypes.Auto,
	FireMode = FireMode.Single,
	BulletType = BulletType.Streak,
	BulletModel = Bullets.Default,
	BulletCache = Caches.DefaultCache
,
	HandleWelds = {
		{	limb = "Right Arm",
			C0 = CFrame.new(0, -0.6, -0.3) * CFrame.Angles(math.rad(-90),math.rad(180),0),
			C1 = CFrame.new()
		},
	},
}
