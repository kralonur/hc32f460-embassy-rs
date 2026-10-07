#[doc = "Register `FEE` writer"]
pub type W = crate::W<FeeSpec>;
#[doc = "Field `FCTOE` writer - desc FCTOE"]
pub type FctoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCCE` writer - desc FCCE"]
pub type FcceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCEBE` writer - desc FCEBE"]
pub type FcebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCIE` writer - desc FCIE"]
pub type FcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTOE` writer - desc FDTOE"]
pub type FdtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCE` writer - desc FDCE"]
pub type FdceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDEBE` writer - desc FDEBE"]
pub type FdebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FACE` writer - desc FACE"]
pub type FaceW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - desc FCTOE"]
    #[inline(always)]
    pub fn fctoe(&mut self) -> FctoeW<'_, FeeSpec> {
        FctoeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc FCCE"]
    #[inline(always)]
    pub fn fcce(&mut self) -> FcceW<'_, FeeSpec> {
        FcceW::new(self, 1)
    }
    #[doc = "Bit 2 - desc FCEBE"]
    #[inline(always)]
    pub fn fcebe(&mut self) -> FcebeW<'_, FeeSpec> {
        FcebeW::new(self, 2)
    }
    #[doc = "Bit 3 - desc FCIE"]
    #[inline(always)]
    pub fn fcie(&mut self) -> FcieW<'_, FeeSpec> {
        FcieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc FDTOE"]
    #[inline(always)]
    pub fn fdtoe(&mut self) -> FdtoeW<'_, FeeSpec> {
        FdtoeW::new(self, 4)
    }
    #[doc = "Bit 5 - desc FDCE"]
    #[inline(always)]
    pub fn fdce(&mut self) -> FdceW<'_, FeeSpec> {
        FdceW::new(self, 5)
    }
    #[doc = "Bit 6 - desc FDEBE"]
    #[inline(always)]
    pub fn fdebe(&mut self) -> FdebeW<'_, FeeSpec> {
        FdebeW::new(self, 6)
    }
    #[doc = "Bit 8 - desc FACE"]
    #[inline(always)]
    pub fn face(&mut self) -> FaceW<'_, FeeSpec> {
        FaceW::new(self, 8)
    }
}
#[doc = "desc FEE\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fee::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeeSpec;
impl crate::RegisterSpec for FeeSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`fee::W`](W) writer structure"]
impl crate::Writable for FeeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FEE to value 0"]
impl crate::Resettable for FeeSpec {}
