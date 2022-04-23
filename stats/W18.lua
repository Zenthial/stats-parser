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
	Name = "W18",
	FullName = "Combat Rifle",
	Category = "Assault",
	Description = "Special operations run by Shock Troopers demanded a more versatile weapon than the W17, therefore it was completely re-designed to produce a shorter-barreled version. The resulting rifle is fitted with an EM scope and a suppressor. This rifle is perfect for Shock Troopers as a soldier can ensure long-range hits and maintain concealment.",
	QuickDescription = "Automatic, Single Shot, Silenced",
	WeaponCost = 2000,
	Slot = 1,
	Holster = Holsters.Back,
	NumHandles = 1,
	NumBarrels = 1,
	CanSprint = true,
	CanCrouch = true,
	HeadshotMultiplier = 2,
	WalkspeedReduce = 0,
	EquipTime = 0.3,
	BatteryDepletionMin = 2,
	BatteryDepletionMax = 3,
	ShotsDeplete = 12,
	MaxSpread = 2,
	MinSpread = 0.25,
	HeatRate = 3,
	CoolTime = 2,
	CoolWait = 0.3,
	CalculateDamage = function(damage, distance)
		return damage
	end,
	VehicleMultiplier = 1,
	FireRate = 11,
	ChargeWait = 0,
	GunType = GunTypes.Auto,
	FireMode = FireMode.Single,
	BulletType = BulletType.Ray,
	BulletModel = Bullets.Default,
	BulletCache = Caches.DefaultCache
,
	HandleWelds = {
		{	limb = "Right Arm",
			C0 = CFrame.new(0, -0.5, -0.25) * CFrame.Angles(math.rad(-90),math.rad(180),0),
			C1 = CFrame.new()
		},
	},
}
