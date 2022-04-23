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
	Name = "SpaceShip",
	FullName = "Charged Railgun",
	Category = "Explosive",
	Description = "It was supposed to be a joke. Then everything went wrong.",
	QuickDescription = "Semi Automatic, Single Explosise Beam",
	WeaponCost = 999999999,
	NumBarrels = 2,
	ShotsDeplete = 80,
	MaxSpread = 10,
	CoolTime = 8,
	VehicleMultiplier = 8,
	CalculateDamage = function(damage, distance)
		damage = damage + (10 * (1/(distance/3)))
		return math.clamp(damage, 20, 35)
	end,
	FireRate = 15,
	ChargeWait = 0.5,
	GunType = GunTypes.Auto,
	FireMode = FireMode.SingleShipExplosive,
	BulletType = BulletType.Ray,
	BulletModel = Bullets.Default,
	BulletCache = Caches.DefaultCache
,
}
