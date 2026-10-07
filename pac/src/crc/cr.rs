#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `CR` reader - desc CR"]
pub type CrR = crate::BitReader;
#[doc = "Field `CR` writer - desc CR"]
pub type CrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFIN` reader - desc REFIN"]
pub type RefinR = crate::BitReader;
#[doc = "Field `REFIN` writer - desc REFIN"]
pub type RefinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFOUT` reader - desc REFOUT"]
pub type RefoutR = crate::BitReader;
#[doc = "Field `REFOUT` writer - desc REFOUT"]
pub type RefoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOROUT` reader - desc XOROUT"]
pub type XoroutR = crate::BitReader;
#[doc = "Field `XOROUT` writer - desc XOROUT"]
pub type XoroutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - desc CR"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc REFIN"]
    #[inline(always)]
    pub fn refin(&self) -> RefinR {
        RefinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc REFOUT"]
    #[inline(always)]
    pub fn refout(&self) -> RefoutR {
        RefoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc XOROUT"]
    #[inline(always)]
    pub fn xorout(&self) -> XoroutR {
        XoroutR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - desc CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<'_, CrSpec> {
        CrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc REFIN"]
    #[inline(always)]
    pub fn refin(&mut self) -> RefinW<'_, CrSpec> {
        RefinW::new(self, 2)
    }
    #[doc = "Bit 3 - desc REFOUT"]
    #[inline(always)]
    pub fn refout(&mut self) -> RefoutW<'_, CrSpec> {
        RefoutW::new(self, 3)
    }
    #[doc = "Bit 4 - desc XOROUT"]
    #[inline(always)]
    pub fn xorout(&mut self) -> XoroutW<'_, CrSpec> {
        XoroutW::new(self, 4)
    }
}
#[doc = "desc CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x1c"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x1c;
}
