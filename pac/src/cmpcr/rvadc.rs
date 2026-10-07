#[doc = "Register `RVADC` reader"]
pub type R = crate::R<RvadcSpec>;
#[doc = "Register `RVADC` writer"]
pub type W = crate::W<RvadcSpec>;
#[doc = "Field `DA1SW` reader - desc DA1SW"]
pub type Da1swR = crate::BitReader;
#[doc = "Field `DA1SW` writer - desc DA1SW"]
pub type Da1swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA2SW` reader - desc DA2SW"]
pub type Da2swR = crate::BitReader;
#[doc = "Field `DA2SW` writer - desc DA2SW"]
pub type Da2swW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFSW` reader - desc VREFSW"]
pub type VrefswR = crate::BitReader;
#[doc = "Field `VREFSW` writer - desc VREFSW"]
pub type VrefswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPRT` reader - desc WPRT"]
pub type WprtR = crate::FieldReader;
#[doc = "Field `WPRT` writer - desc WPRT"]
pub type WprtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - desc DA1SW"]
    #[inline(always)]
    pub fn da1sw(&self) -> Da1swR {
        Da1swR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc DA2SW"]
    #[inline(always)]
    pub fn da2sw(&self) -> Da2swR {
        Da2swR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - desc VREFSW"]
    #[inline(always)]
    pub fn vrefsw(&self) -> VrefswR {
        VrefswR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - desc WPRT"]
    #[inline(always)]
    pub fn wprt(&self) -> WprtR {
        WprtR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc DA1SW"]
    #[inline(always)]
    pub fn da1sw(&mut self) -> Da1swW<'_, RvadcSpec> {
        Da1swW::new(self, 0)
    }
    #[doc = "Bit 1 - desc DA2SW"]
    #[inline(always)]
    pub fn da2sw(&mut self) -> Da2swW<'_, RvadcSpec> {
        Da2swW::new(self, 1)
    }
    #[doc = "Bit 4 - desc VREFSW"]
    #[inline(always)]
    pub fn vrefsw(&mut self) -> VrefswW<'_, RvadcSpec> {
        VrefswW::new(self, 4)
    }
    #[doc = "Bits 8:15 - desc WPRT"]
    #[inline(always)]
    pub fn wprt(&mut self) -> WprtW<'_, RvadcSpec> {
        WprtW::new(self, 8)
    }
}
#[doc = "desc RVADC\n\nYou can [`read`](crate::Reg::read) this register and get [`rvadc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rvadc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RvadcSpec;
impl crate::RegisterSpec for RvadcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rvadc::R`](R) reader structure"]
impl crate::Readable for RvadcSpec {}
#[doc = "`write(|w| ..)` method takes [`rvadc::W`](W) writer structure"]
impl crate::Writable for RvadcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RVADC to value 0"]
impl crate::Resettable for RvadcSpec {}
