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
	Name = "R87",
	FullName = "Charged Railgun",
	Category = "Explosive",
	Description = "The HilTech Type 87 Heavy Assault Rifle is classified as a 'Charged Railgun' which fires ultra-high intensity phaser bolts at its target. The bolts explode on impact with the surface they hit, and due to their higher energy yield can pierce through cover, hitting any unfortunate enemy that is directly behind said cover.",
	QuickDescription = "Semi Automatic, Single Explosise Beam",
	WeaponCost = 5000,
	Slot = 1,
	Holster = Holsters.Back,
	NumHandles = 1,
	NumBarrels = 1,
	CanSprint = true,
	CanCrouch = true,
	HeadshotMultiplier = 2,
	WalkspeedReduce = 0,
	EquipTime = 0.3,
	BatteryDepletionMin = 3,
	BatteryDepletionMax = 4,
	ShotsDeplete = 1,
	MaxSpread = 2,
	MinSpread = 1,
	HeatRate = 35,
	CoolTime = 4,
	CoolWait = 0,
	VehicleMultiplier = 10,
	CalculateDamage = function(damage, distance)
		damage = damage + (10 * (1/(distance/3)))
		return math.clamp(damage, 20, 35)
	end,
	FireRate = 3,
	ChargeWait = 0,
	GunType = GunTypes.Auto,
	FireMode = FireMode.SingleExplosive,
	BulletType = BulletType.Ray,
	BulletModel = Bullets.Default,
	BulletCache = Caches.DefaultCache
,
	HandleWelds = {
		{	limb = "Right Arm",
			C0 = CFrame.new(0, -0.75, -0.25) * CFrame.Angles(math.rad(-90),math.rad(180),0),
			C1 = CFrame.new()
		},
	},
}
