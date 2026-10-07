#[doc = "Register `SLR0` reader"]
pub type R = crate::R<Slr0Spec>;
#[doc = "Register `SLR0` writer"]
pub type W = crate::W<Slr0Spec>;
#[doc = "Field `SLADDR0` reader - desc SLADDR0"]
pub type Sladdr0R = crate::FieldReader<u16>;
#[doc = "Field `SLADDR0` writer - desc SLADDR0"]
pub type Sladdr0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLADDR0EN` reader - desc SLADDR0EN"]
pub type Sladdr0enR = crate::BitReader;
#[doc = "Field `SLADDR0EN` writer - desc SLADDR0EN"]
pub type Sladdr0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDRMOD0` reader - desc ADDRMOD0"]
pub type Addrmod0R = crate::BitReader;
#[doc = "Field `ADDRMOD0` writer - desc ADDRMOD0"]
pub type Addrmod0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - desc SLADDR0"]
    #[inline(always)]
    pub fn sladdr0(&self) -> Sladdr0R {
        Sladdr0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - desc SLADDR0EN"]
    #[inline(always)]
    pub fn sladdr0en(&self) -> Sladdr0enR {
        Sladdr0enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - desc ADDRMOD0"]
    #[inline(always)]
    pub fn addrmod0(&self) -> Addrmod0R {
        Addrmod0R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - desc SLADDR0"]
    #[inline(always)]
    pub fn sladdr0(&mut self) -> Sladdr0W<'_, Slr0Spec> {
        Sladdr0W::new(self, 0)
    }
    #[doc = "Bit 12 - desc SLADDR0EN"]
    #[inline(always)]
    pub fn sladdr0en(&mut self) -> Sladdr0enW<'_, Slr0Spec> {
        Sladdr0enW::new(self, 12)
    }
    #[doc = "Bit 15 - desc ADDRMOD0"]
    #[inline(always)]
    pub fn addrmod0(&mut self) -> Addrmod0W<'_, Slr0Spec> {
        Addrmod0W::new(self, 15)
    }
}
#[doc = "desc SLR0\n\nYou can [`read`](crate::Reg::read) this register and get [`slr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Slr0Spec;
impl crate::RegisterSpec for Slr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slr0::R`](R) reader structure"]
impl crate::Readable for Slr0Spec {}
#[doc = "`write(|w| ..)` method takes [`slr0::W`](W) writer structure"]
impl crate::Writable for Slr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLR0 to value 0x1000"]
impl crate::Resettable for Slr0Spec {
    const RESET_VALUE: u32 = 0x1000;
}
