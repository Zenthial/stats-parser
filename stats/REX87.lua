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
	Name = "REX87",
	FullName = "Charged Railgun",
	Category = "Explosive",
	Description = "It was supposed to be a joke. Then everything went wrong.",
	QuickDescription = "Semi Automatic, Single Explosise Beam",
	WeaponCost = 5000,
	Slot = 1,
	Holster = Holsters.Back,
	NumHandles = 1,
	NumBarrels = 3,
	CanSprint = true,
	CanCrouch = true,
	HeadshotMultiplier = 2,
	WalkspeedReduce = 5,
	EquipTime = 0.5,
	BatteryDepletionMin = 2,
	BatteryDepletionMax = 3,
	ShotsDeplete = 150,
	MaxSpread = 1,
	MinSpread = 0.5,
	HeatRate = 0.75,
	CoolTime = 8,
	CoolWait = 0.3,
	VehicleMultiplier = 500,
	CalculateDamage = function(damage, distance)
		damage = damage + (10 * (1/(distance/3)))
		return math.clamp(damage, 20, 35)
	end,
	FireRate = 15,
	ChargeWait = 0.5,
	GunType = GunTypes.Auto,
	FireMode = FireMode.SingleExplosive,
	BulletType = BulletType.Ray,
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
