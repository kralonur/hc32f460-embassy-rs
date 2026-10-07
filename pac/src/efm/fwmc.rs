#[doc = "Register `FWMC` reader"]
pub type R = crate::R<FwmcSpec>;
#[doc = "Register `FWMC` writer"]
pub type W = crate::W<FwmcSpec>;
#[doc = "Field `PEMODE` reader - desc PEMODE"]
pub type PemodeR = crate::BitReader;
#[doc = "Field `PEMODE` writer - desc PEMODE"]
pub type PemodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEMOD` reader - desc PEMOD"]
pub type PemodR = crate::FieldReader;
#[doc = "Field `PEMOD` writer - desc PEMOD"]
pub type PemodW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUSHLDCTL` reader - desc BUSHLDCTL"]
pub type BushldctlR = crate::BitReader;
#[doc = "Field `BUSHLDCTL` writer - desc BUSHLDCTL"]
pub type BushldctlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PEMODE"]
    #[inline(always)]
    pub fn pemode(&self) -> PemodeR {
        PemodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - desc PEMOD"]
    #[inline(always)]
    pub fn pemod(&self) -> PemodR {
        PemodR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - desc BUSHLDCTL"]
    #[inline(always)]
    pub fn bushldctl(&self) -> BushldctlR {
        BushldctlR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PEMODE"]
    #[inline(always)]
    pub fn pemode(&mut self) -> PemodeW<'_, FwmcSpec> {
        PemodeW::new(self, 0)
    }
    #[doc = "Bits 4:6 - desc PEMOD"]
    #[inline(always)]
    pub fn pemod(&mut self) -> PemodW<'_, FwmcSpec> {
        PemodW::new(self, 4)
    }
    #[doc = "Bit 8 - desc BUSHLDCTL"]
    #[inline(always)]
    pub fn bushldctl(&mut self) -> BushldctlW<'_, FwmcSpec> {
        BushldctlW::new(self, 8)
    }
}
#[doc = "desc FWMC\n\nYou can [`read`](crate::Reg::read) this register and get [`fwmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwmcSpec;
impl crate::RegisterSpec for FwmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwmc::R`](R) reader structure"]
impl crate::Readable for FwmcSpec {}
#[doc = "`write(|w| ..)` method takes [`fwmc::W`](W) writer structure"]
impl crate::Writable for FwmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FWMC to value 0"]
impl crate::Resettable for FwmcSpec {}
